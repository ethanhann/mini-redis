use std::path::Path;

use confval::format::toml::parse_toml;
use confval::prelude::*;

use crate::Error;

use super::parse::ConfigFile;
use super::types::ServerConfig;

#[derive(Default, Clone)]
pub struct ConfigOverrides {
    pub hostname: Option<String>,
    pub port: Option<u16>,
}

pub fn load_config(path: &Path, overrides: &ConfigOverrides) -> Result<ServerConfig, Error> {
    if !path.exists() {
        return Err(format!("config file does not exist: {}", path.display()).into());
    }

    let text = std::fs::read_to_string(path)?;

    //-------------------------------------------------------------------------
    // Parse (structural): the frontend builds a span-carrying field model and
    // the derived `FromFields` walk turns it into the spec.
    //-------------------------------------------------------------------------

    let mut sources = SourceMap::new();
    let id = sources.add(path.to_string_lossy(), text);
    let mut report = Report::new();

    let parsed: Option<ConfigFile> = parse_toml(&sources, id, &mut report);

    let resolved = parsed.and_then(|mut file| {
        //---------------------------------------------------------------------
        // Apply CLI overrides on the parsed spec. A detached span marks a value
        // that did not come from the source file.
        //---------------------------------------------------------------------

        if let Some(hostname) = &overrides.hostname {
            file.server.value.hostname = Located::detached(hostname.clone());
        }
        if let Some(port) = overrides.port {
            file.server.value.port = Located::detached(port as i64);
        }

        //---------------------------------------------------------------------
        // Validate (semantic): ranges, closed sets, and cross-field rules,
        // each reported at the span its field already carries.
        //---------------------------------------------------------------------

        file.server.value.validate(&mut report);

        //---------------------------------------------------------------------
        // Gate, then lower. Narrowing in the lowering functions is safe only
        // because lowering does not run on a report that has errors.
        //---------------------------------------------------------------------

        if report.has_errors() {
            return None;
        }

        ServerConfig::lower(&file.server.value, &mut report)
    });

    if report.has_issues() {
        let mut out = String::new();
        report.render_pretty(&sources, &mut out)?;
        eprint!("{out}");
    }

    resolved.ok_or_else(|| "config validation failed".into())
}
