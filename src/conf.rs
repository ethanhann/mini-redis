use crate::Error;
use confval::{
    range_constraint, validate_range_field, RangeConstraint, SimpleOrigin, ValidateSpec,
    ValidationReport,
};
use std::convert::TryFrom;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;

//-----------------------------------------------------------------------------
// Acquisition
//-----------------------------------------------------------------------------

#[derive(Deserialize)]
struct ConfigFile {
    server: ServerSpec,
}

#[derive(Default)]
pub struct ConfigOverrides {
    pub hostname: Option<String>,
    pub port: Option<u16>,
}

pub struct ResolvedConfig {
    pub server: ServerConfig,
    pub report: Option<ValidationReport<SimpleOrigin>>,
}

pub fn load_config(path: PathBuf, overrides: ConfigOverrides) -> Result<ResolvedConfig, Error> {
    if !path.exists() {
        return Err(format!("config file does not exist: {}", path.display()).into());
    }

    let file = parse_config_file(&path)?;
    let mut spec = file.server;

    if let Some(hostname) = overrides.hostname {
        spec.hostname = hostname;
    }
    if let Some(port) = overrides.port {
        spec.port = port;
    }

    let origin = SimpleOrigin::new(path.to_string_lossy(), "server");
    let mut report = ValidationReport::default();
    spec.validate(&origin, &mut report);

    if report.has_issues() {
        let mut out = String::new();
        report.render_plain(&mut out)?;
        eprint!("{out}");
    }

    if report.has_errors() {
        return Err("config validation failed".into());
    }

    lower_config(spec, report)
}

fn parse_config_file(path: &PathBuf) -> Result<ConfigFile, Error> {
    let contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}

//-----------------------------------------------------------------------------
// Validation
//-----------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct ServerSpec {
    pub hostname: String,
    pub port: u16,

    /// Maximum number of concurrent connections the redis server will accept.
    ///
    /// When this limit is reached, the server will stop accepting connections until
    /// an active connection terminates.
    ///
    /// A real application will want to make this value configurable, but for this
    /// example, it is hard coded.
    ///
    /// This is also set to a pretty low value to discourage using this in
    /// production (you'd think that all the disclaimers would make it obvious that
    /// this is not a serious project... but I thought that about mini-http as
    /// well).
    pub max_connections: usize,
}

range_constraint!(PORT_RANGE, u16, min: 1, max: 65535);
range_constraint!(MAX_CONNECTIONS_RANGE, usize, min: 1, max: 250, help: "A friendly reminder that this is not a production server ;)");

impl ValidateSpec<SimpleOrigin> for ServerSpec {
    fn validate(&self, origin: &SimpleOrigin, report: &mut ValidationReport<SimpleOrigin>) {
        validate_range_field!(PORT_RANGE, self.port, report, origin);
        validate_range_field!(MAX_CONNECTIONS_RANGE, self.max_connections, report, origin);

        if self.hostname.is_empty() {
            report.push(issues::hostname_empty(origin));
        }
    }
}

mod issues {
    use confval::{SimpleOrigin, ValidationIssue};

    pub(super) fn hostname_empty(origin: &SimpleOrigin) -> ValidationIssue<SimpleOrigin> {
        ValidationIssue::error_with_help(
            "hostname cannot be empty",
            origin.clone(),
            "Set hostname to a valid DNS name or IP address.",
        )
    }
}

//-----------------------------------------------------------------------------
// Lowering
//-----------------------------------------------------------------------------

pub struct ServerConfig {
    pub addr: SocketAddr,
    pub max_connections: usize,
}

impl TryFrom<ServerSpec> for ServerConfig {
    type Error = Error;

    fn try_from(spec: ServerSpec) -> Result<Self, Self::Error> {
        let addr: SocketAddr = format!("{}:{}", spec.hostname, spec.port).parse()?;
        Ok(ServerConfig {
            addr,
            max_connections: spec.max_connections as usize,
        })
    }
}

fn lower_config(
    spec: ServerSpec,
    report: ValidationReport<SimpleOrigin>,
) -> Result<ResolvedConfig, Error> {
    let has_warnings = report.has_warnings();
    let server = ServerConfig::try_from(spec)?;
    Ok(ResolvedConfig {
        server,
        report: if has_warnings { Some(report) } else { None },
    })
}
