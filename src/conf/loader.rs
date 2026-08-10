//! Turning a configuration file into a [`ServerConfig`].
//!
//! [`load`] runs the four phases in order. Parsing reads the file into the
//! spec, validation reports what the values mean, the error gate stops the run
//! while anything is wrong, and lowering produces the runtime type. Every
//! failure along the way is a reported diagnostic rather than a panic, so a
//! misconfigured server declines to start instead of crashing.

use crate::conf::types::runtime::server::ServerConfig;
use crate::conf::types::specification::server::ServerSpec;
use confval::prelude::{Lower, Report, SourceMap, Validate};
use std::fs;
use std::path::Path;

/// Loads the configuration, taking every default when `path` is `None`.
///
/// Returns `None` once anything has been reported as an error, having already
/// written the diagnostics to standard error. Warnings are printed and the
/// load continues, so a privileged port starts the server with a note rather
/// than refusing to run.
pub fn load(path: Option<&Path>) -> Option<ServerConfig> {
    let mut sources = SourceMap::new();
    let mut report = Report::new();

    let spec = read_spec(path, &mut sources, &mut report);

    if let Some(spec) = &spec {
        spec.validate_all(&mut report);
    }

    // The gate. Lowering narrows values on the assumption that validation
    // accepted them, so it must not run while anything is wrong.
    let config = if report.has_errors() {
        None
    } else {
        spec.as_ref()
            .and_then(|spec| ServerConfig::lower(spec, &mut report))
    };

    render(&report, &sources);

    if report.has_errors() {
        None
    } else {
        config
    }
}

/// Reads and parses the file, or produces the all-defaults spec when there is
/// no file to read.
///
/// A file that cannot be read and a file that does not parse both land in the
/// report, so the caller has one place to look for the reason.
fn read_spec(
    path: Option<&Path>,
    sources: &mut SourceMap,
    report: &mut Report,
) -> Option<ServerSpec> {
    let Some(path) = path else {
        return Some(ServerSpec::default());
    };

    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            report
                .error(format!("cannot read {}: {}", path.display(), error))
                .emit();
            return None;
        }
    };

    let id = sources.add(path.display().to_string(), text);
    confval::format::toml::parse_toml(sources, id, report)
}

/// Writes the diagnostics to standard error.
///
/// This runs before logging is set up, so it writes directly rather than
/// through `tracing`.
fn render(report: &Report, sources: &SourceMap) {
    if !report.has_issues() {
        return;
    }

    let mut out = String::new();
    if report.render_pretty(sources, &mut out).is_ok() {
        eprint!("{out}");
    }
}
