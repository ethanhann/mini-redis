use mini_redis::conf::{self, LoadError, LogLevel};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

#[test]
fn a_complete_fixture_lowers_to_the_runtime_values() {
    // Arrange
    let path = "tests/fixtures/server.toml";

    // Act
    let config = conf::load(path).expect("the fixture is valid");

    // Assert
    assert_eq!(config.bind, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    assert_eq!(config.port, 7777);
    assert_eq!(config.max_connections, 32);
    assert_eq!(config.log_level, LogLevel::Debug);
    assert_eq!(config.backoff.initial, Duration::from_secs(2));
    assert_eq!(config.backoff.max, Duration::from_secs(16));
}

#[test]
fn an_omitted_field_takes_the_declared_default() {
    // Arrange
    let path = "tests/fixtures/server_partial.toml";

    // Act
    let config = conf::load(path).expect("the fixture is valid");

    // Assert
    assert_eq!(config.port, 7000);
    assert_eq!(config.bind, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(config.max_connections, 250);
    assert_eq!(config.log_level, LogLevel::Info);
    assert_eq!(config.backoff.initial, Duration::from_secs(1));
    assert_eq!(config.backoff.max, Duration::from_secs(64));
}

#[test]
fn the_declared_defaults_lower_without_a_file() {
    // Arrange
    let expected_bind = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // Act
    let config = conf::defaults().expect("the declared defaults are valid");

    // Assert
    assert_eq!(config.bind, expected_bind);
    assert_eq!(config.port, 6379);
    assert_eq!(config.max_connections, 250);
    assert_eq!(config.log_level, LogLevel::Info);
    assert_eq!(config.backoff.initial, Duration::from_secs(1));
    assert_eq!(config.backoff.max, Duration::from_secs(64));
}

#[test]
fn a_bad_fixture_reports_every_problem_at_once() {
    // Arrange
    let path = "tests/fixtures/server_invalid.toml";

    // Act
    let error = conf::load(path).expect_err("the fixture is not valid");

    // Assert
    // Every rule appends to the same report, so one load reports the address,
    // the port, the connection limit, the log level, and the cross-field
    // backoff rule together rather than stopping at the first.
    let LoadError::Invalid { messages, .. } = &error else {
        panic!("expected an invalid configuration, found {:?}", error);
    };
    assert_eq!(messages.len(), 5, "messages: {:?}", messages);
    assert!(
        messages.iter().any(|m| m.contains("bind")),
        "{:?}",
        messages
    );
    assert!(
        messages.iter().any(|m| m.contains("port")),
        "{:?}",
        messages
    );
    assert!(
        messages.iter().any(|m| m.contains("max_connections")),
        "{:?}",
        messages
    );
    assert!(
        messages.iter().any(|m| m.contains("log_level")),
        "{:?}",
        messages
    );
    assert!(
        messages.iter().any(|m| m.contains("initial_seconds")),
        "{:?}",
        messages
    );
}

#[test]
fn a_syntax_error_reports_rather_than_panicking() {
    // Arrange
    let text = "port = ";

    // Act
    let error = conf::load_text("broken.toml", text).expect_err("the text is not valid TOML");

    // Assert
    let LoadError::Invalid { messages, .. } = &error else {
        panic!("expected an invalid configuration, found {:?}", error);
    };
    assert!(!messages.is_empty());
}

#[test]
fn a_missing_file_reports_the_path() {
    // Arrange
    let path = "tests/fixtures/absent.toml";

    // Act
    let error = conf::load(path).expect_err("the file does not exist");

    // Assert
    let LoadError::Read { path: reported, .. } = &error else {
        panic!("expected a read failure, found {:?}", error);
    };
    assert_eq!(reported.to_str(), Some(path));
}
