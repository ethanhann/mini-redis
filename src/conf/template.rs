use confval::format::toml::emit_toml;
use confval::prelude::*;

use crate::Error;

use super::parse::ConfigFile;
use super::types::ServerSpec;

/// Render a documented configuration file from the spec types.
///
/// The field names, defaults, and doc comments all come from `ServerSpec`, so
/// the template cannot drift from what the parser accepts.
///
/// A field with no value, such as `pid_file`, is rendered commented out rather
/// than dropped, so the file still shows that the setting exists. Uncomment the
/// line to turn it on.
pub fn render_template() -> Result<String, Error> {
    let file = ConfigFile {
        server: Located::detached(ServerSpec::default()),
    };

    emit_toml(&file.to_template()).map_err(|error| error.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_documents_every_field_the_spec_accepts() {
        // Arrange
        let expected = [
            "hostname",
            "port",
            "max_connections",
            "shutdown_timeout_secs",
            "pub_sub_channel_capacity",
            "pid_file",
        ];

        // Act
        let text = render_template().unwrap();

        // Assert
        for field in expected {
            assert!(
                text.contains(field),
                "expected the template to mention {}, got:\n{}",
                field,
                text
            );
        }
        assert!(text.contains("[server]"), "got:\n{}", text);
    }

    #[test]
    fn template_carries_the_defaults_the_parser_fills() {
        // Act
        let text = render_template().unwrap();

        // Assert
        assert!(text.contains("hostname = \"127.0.0.1\""), "got:\n{}", text);
        assert!(text.contains("port = 6379"), "got:\n{}", text);
        assert!(text.contains("max_connections = 250"), "got:\n{}", text);
        assert!(
            text.contains("shutdown_timeout_secs = 30"),
            "got:\n{}",
            text
        );
        assert!(
            text.contains("pub_sub_channel_capacity = 1024"),
            "got:\n{}",
            text
        );
    }

    #[test]
    fn template_renders_an_unset_field_commented_out() {
        // Act
        let text = render_template().unwrap();

        // Assert
        assert!(
            text.contains("#pid_file"),
            "expected pid_file to be commented out, got:\n{}",
            text
        );
        assert!(
            text.contains("# Path to write the server's PID file."),
            "expected pid_file's doc comment above it, got:\n{}",
            text
        );
    }

    #[test]
    fn template_parses_back_into_the_same_spec() {
        // Arrange
        let text = render_template().unwrap();
        let mut sources = SourceMap::new();
        let id = sources.add("template.toml", text.clone());
        let mut report = Report::new();

        // Act
        let parsed: Option<ConfigFile> =
            confval::format::toml::parse_toml(&sources, id, &mut report);

        // Assert
        let parsed = parsed.expect("the generated template parses");
        assert!(!report.has_errors(), "got:\n{}", text);
        assert_eq!(parsed.server.value.hostname.value, "127.0.0.1");
        assert_eq!(parsed.server.value.port.value, 6379);
        assert_eq!(parsed.server.value.max_connections.value, 250);
        assert_eq!(parsed.server.value.shutdown_timeout_secs.value, 30);
        assert_eq!(parsed.server.value.pub_sub_channel_capacity.value, 1024);
        assert!(
            parsed.server.value.pid_file.is_none(),
            "the commented entry stays off"
        );
    }
}
