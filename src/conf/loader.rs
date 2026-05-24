use crate::Error;
use confval::{SimpleOrigin, ValidateSpec, ValidationReport};
use std::path::Path;

use super::lower::lower_config;
use super::parse::parse_config_file;

#[derive(Default, Clone)]
pub struct ConfigOverrides {
    pub hostname: Option<String>,
    pub port: Option<u16>,
}

pub struct ResolvedConfig {
    pub server: super::types::ServerConfig,
    pub client: super::types::ClientConfig,
}

pub fn load_config(path: &Path, overrides: &ConfigOverrides) -> Result<ResolvedConfig, Error> {
    if !path.exists() {
        return Err(format!("config file does not exist: {}", path.display()).into());
    }

    //-------------------------------------------------------------------------
    // Parse
    //-------------------------------------------------------------------------

    let file = parse_config_file(path)?;
    let mut server_spec = file.server;
    let client_spec = file.client;

    if let Some(ref hostname) = overrides.hostname {
        server_spec.hostname = hostname.clone();
    }
    if let Some(port) = overrides.port {
        server_spec.port = port;
    }

    let origin = SimpleOrigin::new(path.to_string_lossy(), "server");

    //-------------------------------------------------------------------------
    // Validate
    //-------------------------------------------------------------------------

    let mut report = ValidationReport::default();
    server_spec.validate(&origin, &mut report);

    let client_origin = SimpleOrigin::new(path.to_string_lossy(), "client");
    client_spec.validate(&client_origin, &mut report);

    if report.has_issues() {
        let mut out = String::new();
        report.render_pretty(&mut out)?;
        eprint!("{out}");
    }

    if report.has_errors() {
        return Err("config validation failed".into());
    }

    //-------------------------------------------------------------------------
    // Lower
    //-------------------------------------------------------------------------

    lower_config(server_spec, client_spec)
}
