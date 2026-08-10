//! mini-redis server.
//!
//! This file is the entry point for the server implemented in the library. It
//! performs command line parsing, loads the configuration file, and passes the
//! result on to `mini_redis::server`.
//!
//! The `clap` crate is used for parsing arguments. A flag the operator passed
//! wins over the same setting in the file, so an existing invocation keeps
//! working once a file is added.

use mini_redis::{conf, server, LogLevel};

use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::EnvFilter;

#[cfg(feature = "otel")]
// To be able to set the XrayPropagator
use opentelemetry::global;
#[cfg(feature = "otel")]
// To configure certain options such as sampling rate
use opentelemetry::sdk::trace as sdktrace;
#[cfg(feature = "otel")]
// For passing along the same XrayId across services
use opentelemetry_aws::trace::XrayPropagator;
#[cfg(feature = "otel")]
// The `Ext` traits are to allow the Registry to accept the
// OpenTelemetry-specific types (such as `OpenTelemetryLayer`)
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, util::TryInitError};

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    let cli = Cli::parse();

    // Loading comes before logging, because the level to log at is one of the
    // settings being loaded. Any diagnostic is written to standard error
    // directly, so it is readable whether or not logging ever starts.
    let Some(config) = conf::load(cli.config.as_deref()) else {
        std::process::exit(1);
    };

    set_up_logging(config.log_level)?;

    let port = cli.port.unwrap_or(config.port);

    // Bind a TCP listener
    let listener = TcpListener::bind(SocketAddr::new(config.bind_address, port)).await?;

    server::run_with_limits(listener, config.limits, signal::ctrl_c()).await;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "mini-redis-server", version, author, about = "A Redis server")]
struct Cli {
    /// Path to a TOML configuration file. Without it, every setting takes its
    /// default.
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Port to listen on. Takes precedence over the configuration file.
    #[arg(long)]
    port: Option<u16>,
}

/// Builds the filter logging starts with.
///
/// `RUST_LOG` wins when it is set, which keeps the usual way of turning up
/// verbosity for one run working without editing the file.
fn env_filter(level: LogLevel) -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.as_str()))
}

#[cfg(not(feature = "otel"))]
fn set_up_logging(level: LogLevel) -> mini_redis::Result<()> {
    // See https://docs.rs/tracing for more info
    tracing_subscriber::fmt()
        .with_env_filter(env_filter(level))
        .try_init()
}

#[cfg(feature = "otel")]
fn set_up_logging(level: LogLevel) -> Result<(), TryInitError> {
    // Set the global propagator to X-Ray propagator
    // Note: If you need to pass the x-amzn-trace-id across services in the same trace,
    // you will need this line. However, this requires additional code not pictured here.
    // For a full example using hyper, see:
    // https://github.com/open-telemetry/opentelemetry-rust/blob/v0.19.0/examples/aws-xray/src/server.rs#L14-L26
    global::set_text_map_propagator(XrayPropagator::default());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            sdktrace::config()
                .with_sampler(sdktrace::Sampler::AlwaysOn)
                // Needed in order to convert the trace IDs into an Xray-compatible format
                .with_id_generator(sdktrace::XrayIdGenerator::default()),
        )
        .install_simple()
        .expect("Unable to initialize OtlpPipeline");

    // Create a tracing layer with the configured tracer
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let filter = env_filter(level);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(filter)
        .with(fmt::Layer::default())
        .try_init()
}
