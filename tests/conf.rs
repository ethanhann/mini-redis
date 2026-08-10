//! Round trips through the configuration pipeline.
//!
//! Each test runs the phases the server runs, so what these assert is what an
//! operator gets.

use confval::format::toml::parse_toml;
use confval::prelude::{Lower, Report, SourceMap, Validate};
use mini_redis::conf::{self, LogLevel, ServerConfig, ServerSpec};
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

/// Parses and validates `text`, returning the spec alongside the report so a
/// test can assert on either.
fn parse(text: &str) -> (SourceMap, Report, Option<ServerSpec>) {
    let mut sources = SourceMap::new();
    let mut report = Report::new();
    let id = sources.add("mini-redis.toml", text);
    let spec: Option<ServerSpec> = parse_toml(&sources, id, &mut report);

    if let Some(spec) = &spec {
        spec.validate_all(&mut report);
    }

    (sources, report, spec)
}

/// Writes `text` to a uniquely named file under the system temp directory and
/// returns the path.
fn fixture_file(name: &str, text: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("mini-redis-conf-{name}.toml"));
    fs::write(&path, text).expect("the temp directory is writable");
    path
}

#[test]
fn a_complete_file_lowers_to_the_values_it_names() {
    // Arrange
    let text = "\
bind_address = \"0.0.0.0\"
port = 7000
log_level = \"debug\"

[limits]
max_connections = 32
max_backoff_seconds = 8
";
    let (_sources, mut report, spec) = parse(text);
    let spec = spec.expect("the fixture is well formed TOML");

    // Act
    let config = ServerConfig::lower(&spec, &mut report);

    // Assert
    let config = config.expect("every value is in range, so lowering succeeds");
    assert!(!report.has_errors(), "a valid file reports no errors");
    assert_eq!(config.bind_address, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    assert_eq!(config.port, 7000);
    assert_eq!(config.log_level, LogLevel::Debug);
    assert_eq!(config.limits.max_connections, 32);
    assert_eq!(config.limits.max_backoff_seconds, 8);
}

#[test]
fn an_omitted_limits_block_lowers_to_the_declared_defaults() {
    // Arrange
    let text = "port = 7000\n";
    let (_sources, mut report, spec) = parse(text);
    let spec = spec.expect("the fixture is well formed TOML");

    // Act
    let config = ServerConfig::lower(&spec, &mut report);

    // Assert
    let config = config.expect("the defaults are in range, so lowering succeeds");
    assert!(
        !report.has_errors(),
        "omitting an optional block is not an error"
    );
    assert!(
        spec.limits.is_none(),
        "the spec records only what the file wrote"
    );
    assert_eq!(config.bind_address, IpAddr::V4(Ipv4Addr::LOCALHOST));
    assert_eq!(config.log_level, LogLevel::Info);
    assert_eq!(config.limits.max_connections, 250);
    assert_eq!(config.limits.max_backoff_seconds, 64);
}

#[test]
fn a_bad_file_reports_every_problem_at_once() {
    // Arrange
    let text = "\
bind_address = \"not-an-address\"
port = 99999
log_level = \"chatty\"

[limits]
max_connections = 0
max_backoff_seconds = 100000
";

    // Act
    let (_sources, report, _spec) = parse(text);

    // Assert
    // Validation accumulates rather than stopping at the first violation, so
    // one run names all five bad values instead of only `bind_address`.
    let messages: Vec<&str> = report
        .issues()
        .iter()
        .map(|issue| issue.message.as_str())
        .collect();
    assert!(report.has_errors());
    for field in [
        "bind_address",
        "port",
        "log_level",
        "max_connections",
        "max_backoff_seconds",
    ] {
        assert!(
            messages.iter().any(|message| message.contains(field)),
            "expected an issue naming {}, got {:?}",
            field,
            messages
        );
    }
    assert!(
        report.issues().iter().all(|issue| issue.span.is_some()),
        "every issue points at the value that caused it"
    );
}

#[test]
fn a_privileged_port_is_a_warning_rather_than_an_error() {
    // Arrange
    let text = "port = 80\n";

    // Act
    let (_sources, report, _spec) = parse(text);

    // Assert
    assert!(!report.has_errors(), "port 80 is legal, just privileged");
    assert!(report.has_warnings());
    assert!(report.issues()[0].message.contains("privileged"));
}

#[test]
fn loading_without_a_path_takes_every_default() {
    // Arrange
    // Nothing to arrange. The absent path is the case under test.

    // Act
    let config = conf::load(None);

    // Assert
    let config = config.expect("the declared defaults load without a file");
    assert_eq!(config.bind_address, IpAddr::V4(Ipv4Addr::LOCALHOST));
    assert_eq!(config.port, 6379);
    assert_eq!(config.log_level, LogLevel::Info);
    assert_eq!(config.limits.max_connections, 250);
    assert_eq!(config.limits.max_backoff_seconds, 64);
}

#[test]
fn loading_a_file_reads_the_values_it_names() {
    // Arrange
    let path = fixture_file("valid", "bind_address = \"::1\"\nport = 7001\n");

    // Act
    let config = conf::load(Some(&path));

    // Assert
    let config = config.expect("the fixture is valid");
    assert_eq!(config.bind_address, "::1".parse::<IpAddr>().unwrap());
    assert_eq!(config.port, 7001);
    assert_eq!(config.limits.max_connections, 250);
}

#[test]
fn loading_a_file_with_a_bad_value_yields_no_config() {
    // Arrange
    let path = fixture_file("out-of-range", "port = 99999\n");

    // Act
    let config = conf::load(Some(&path));

    // Assert
    // The gate stops the load before lowering, so the caller gets nothing to
    // start a server with.
    assert!(config.is_none());
}

#[test]
fn loading_a_missing_file_yields_no_config() {
    // Arrange
    let path = std::env::temp_dir().join("mini-redis-conf-does-not-exist.toml");
    let _ = fs::remove_file(&path);

    // Act
    let config = conf::load(Some(&path));

    // Assert
    // An unreadable file is reported rather than silently falling back to the
    // defaults, because the operator asked for that file.
    assert!(config.is_none());
}
