use std::path::Path;

use confval::format::toml::{emit_toml, parse_toml};
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
    with_validated_spec(path, overrides, |file, report| {
        //-------------------------------------------------------------------------
        // Lower. Narrowing in the lowering functions is safe only because
        // lowering does not run on a report with errors.
        //-------------------------------------------------------------------------
        ServerConfig::lower(&file.server.value, report)
    })
}

/// Read `path`, apply `overrides`, validate the result, and hand the spec to
/// `finish` when no error was reported.
///
/// Everything up to the error gate is the same whether the caller wants the
/// lowered config or one of the views, so both go through here. `finish` runs
/// before the report is rendered, so a diagnostic it adds is still shown.
fn with_validated_spec<T>(
    path: &Path,
    overrides: &ConfigOverrides,
    finish: impl FnOnce(&ConfigFile, &mut Report) -> Option<T>,
) -> Result<T, Error> {
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
        // that did not come from the source file. That is also why an override
        // does not show up in the source view.
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

        if report.has_errors() {
            return None;
        }

        finish(&file, &mut report)
    });

    if report.has_issues() {
        let mut out = String::new();
        report.render_pretty(&sources, &mut out)?;
        eprint!("{out}");
    }

    resolved.ok_or_else(|| "config validation failed".into())
}

/// Render the configuration `path` resolves to, as TOML.
///
/// [`ConfigView::Source`] shows what the file set, with every default left out,
/// which is what you want when tracking down where a value came from.
/// [`ConfigView::Populated`] fills the defaults in, so it shows what the server
/// resolved to.
pub fn render_config(
    path: &Path,
    overrides: &ConfigOverrides,
    view: ConfigView,
) -> Result<String, Error> {
    with_validated_spec(path, overrides, |file, report| {
        let fields = match view {
            ConfigView::Source => file.to_source_fields(),
            ConfigView::Populated => file.to_fields(),
        };
        match emit_toml(&fields) {
            Ok(text) => Some(text),
            Err(error) => {
                report
                    .error(format!("cannot render the configuration: {error}"))
                    .emit();
                None
            }
        }
    })
}

/// Which form of the loaded configuration to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigView {
    /// Only what the file set. Defaults and CLI overrides are left out.
    Source,
    /// Every setting, with the defaults the file omitted filled in.
    Populated,
}
