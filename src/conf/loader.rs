//! The entry point that runs the four phases in order.
//!
//! Parsing produces the spec, validation fills the report, the gate stops the
//! load while the report holds errors, and lowering produces the runtime
//! configuration. The path never panics. A misconfiguration returns a
//! `LoadError` that carries the rendered diagnostics.

use confval::prelude::*;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::conf::types::runtime::server::ServerConfig;
use crate::conf::types::specification::server::ServerSpec;

/// Why a configuration load did not produce a configuration.
#[derive(Debug)]
pub enum LoadError {
    /// The file could not be read.
    Read {
        /// The path that was read.
        path: PathBuf,
        /// The underlying failure.
        source: io::Error,
    },
    /// The file was read, and the report holds at least one error.
    Invalid {
        /// One message for each reported problem, in report order.
        messages: Vec<String>,
        /// The rendered report, ready to print.
        diagnostics: String,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Read { path, source } => {
                write!(f, "cannot read {}: {}", path.display(), source)
            }
            LoadError::Invalid { diagnostics, .. } => write!(f, "{}", diagnostics.trim_end()),
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Read { source, .. } => Some(source),
            LoadError::Invalid { .. } => None,
        }
    }
}

/// Reads a configuration file and returns the runtime configuration.
pub fn load(path: impl AsRef<Path>) -> Result<ServerConfig, LoadError> {
    let path = path.as_ref();
    let text = fs::read_to_string(path).map_err(|source| LoadError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    load_text(&path.display().to_string(), &text)
}

/// Runs the four phases over configuration text that is already in memory.
///
/// The name labels the source in a diagnostic.
pub fn load_text(name: &str, text: &str) -> Result<ServerConfig, LoadError> {
    let mut sources = SourceMap::new();
    let mut report = Report::new();
    let id = sources.add(name, text);

    let spec: Option<ServerSpec> = confval::format::toml::parse_toml(&sources, id, &mut report);

    let Some(spec) = spec else {
        return Err(invalid(&report, &sources));
    };

    spec.validate_all(&mut report);

    if report.has_errors() {
        return Err(invalid(&report, &sources));
    }

    match ServerConfig::lower(&spec, &mut report) {
        Some(config) => Ok(config),
        None => Err(invalid(&report, &sources)),
    }
}

/// Returns the configuration the spec declares when no file is supplied.
pub fn defaults() -> Result<ServerConfig, LoadError> {
    let sources = SourceMap::new();
    let mut report = Report::new();
    let spec = ServerSpec::default();

    spec.validate_all(&mut report);

    if report.has_errors() {
        return Err(invalid(&report, &sources));
    }

    match ServerConfig::lower(&spec, &mut report) {
        Some(config) => Ok(config),
        None => Err(invalid(&report, &sources)),
    }
}

fn invalid(report: &Report, sources: &SourceMap) -> LoadError {
    let messages = report
        .issues()
        .iter()
        .map(|issue| issue.message.clone())
        .collect();

    let mut diagnostics = String::new();
    if report.render_pretty(sources, &mut diagnostics).is_err() {
        diagnostics.push_str("the configuration is not valid, and the report could not render");
    }

    LoadError::Invalid {
        messages,
        diagnostics,
    }
}
