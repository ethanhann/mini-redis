use crate::Error;
use confval::{SimpleOrigin, ValidateSpec, ValidationReport};
use std::path::PathBuf;

use super::lower::lower_config;
use super::parse::parse_config_file;
use super::types::runtime::client_config::ClientConfig;
use super::types::runtime::server_config::ServerConfig;

#[derive(Default)]
pub struct ConfigOverrides {
    pub hostname: Option<String>,
    pub port: Option<u16>,
}

pub struct ResolvedConfig {
    pub server: ServerConfig,
    pub client: ClientConfig,
    pub report: Option<ValidationReport<SimpleOrigin>>,
}

pub fn load_config(path: PathBuf, overrides: ConfigOverrides) -> Result<ResolvedConfig, Error> {
    if !path.exists() {
        return Err(format!("config file does not exist: {}", path.display()).into());
    }

    let file = parse_config_file(&path)?;
    let mut server_spec = file.server;
    let client_spec = file.client;

    if let Some(hostname) = overrides.hostname {
        server_spec.hostname = hostname;
    }
    if let Some(port) = overrides.port {
        server_spec.port = port;
    }

    let origin = SimpleOrigin::new(path.to_string_lossy(), "server");
    let mut report = ValidationReport::default();
    server_spec.validate(&origin, &mut report);

    let client_origin = SimpleOrigin::new(path.to_string_lossy(), "client");
    client_spec.validate(&client_origin, &mut report);

    if report.has_issues() {
        let mut out = String::new();
        report.render_plain(&mut out)?;
        eprint!("{out}");
    }

    if report.has_errors() {
        return Err("config validation failed".into());
    }

    lower_config(server_spec, client_spec, report)
}
