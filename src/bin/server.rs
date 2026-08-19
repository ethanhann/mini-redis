//! mini-redis server.
//!
//! This file is the entry point for the server implemented in the library. It
//! performs command line parsing and passes the arguments on to
//! `mini_redis::server`.
//!
//! The `clap` crate is used for parsing arguments. The `confval` pipeline in
//! `mini_redis::conf` reads the optional configuration file. A flag given on
//! the command line overrides the matching value in that file.

use mini_redis::conf::{self, LoadError, LogLevel, ServerConfig};
use mini_redis::server;

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

    // A misconfiguration is the operator's to fix, so print the diagnostics and
    // stop rather than returning an error that renders through `Debug`.
    let config = match resolve_config(&cli) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    set_up_logging(config.log_level)?;

    // Bind a TCP listener
    let address = SocketAddr::new(config.bind, config.port);
    let listener = TcpListener::bind(address).await?;

    server::run_with_config(listener, signal::ctrl_c(), config).await;

    Ok(())
}

/// Read the configuration file when one is given, then apply the flags over it.
///
/// A flag wins over the file, and the file wins over the declared defaults.
fn resolve_config(cli: &Cli) -> Result<ServerConfig, LoadError> {
    let mut config = match &cli.config {
        Some(path) => conf::load(path)?,
        None => conf::defaults()?,
    };

    if let Some(port) = cli.port {
        config.port = port;
    }

    Ok(config)
}

#[derive(Parser, Debug)]
#[command(name = "mini-redis-server", version, author, about = "A Redis server")]
struct Cli {
    #[arg(long)]
    port: Option<u16>,

    /// Path to a TOML configuration file.
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,
}

#[cfg(not(feature = "otel"))]
fn set_up_logging(level: LogLevel) -> mini_redis::Result<()> {
    // See https://docs.rs/tracing for more info
    tracing_subscriber::fmt()
        .with_env_filter(filter(level))
        .try_init()
}

/// Read the filter from `RUST_LOG`, and fall back to the configured level.
fn filter(level: LogLevel) -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.as_str()))
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

    let filter = filter(level);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(filter)
        .with(fmt::Layer::default())
        .try_init()
}
