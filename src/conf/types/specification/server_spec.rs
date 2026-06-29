use confval::diagnostic::Report;
use confval::pipeline::Validate;
use confval::source::Located;
use confval::{range_constraint, RangeConstraint};

use crate::conf::validation::validator::{net, path};

#[derive(confval::Spec)]
pub struct ServerSpec {
    #[confval(default = "127.0.0.1".to_string())]
    pub hostname: Located<String>,

    #[confval(default = 6379)]
    pub port: Located<i64>,

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
    #[confval(default = 250)]
    pub max_connections: Located<i64>,

    #[confval(default = 30)]
    pub shutdown_timeout_secs: Located<i64>,

    /// Capacity of each pub/sub broadcast channel. A message is stored in the
    /// channel until all subscribers have seen it, so a slow subscriber could
    /// result in messages being held indefinitely. When the channel's capacity
    /// fills up, publishing will result in old messages being dropped. This
    /// prevents slow consumers from blocking the entire system.
    #[confval(default = 1024)]
    pub pub_sub_channel_capacity: Located<i64>,

    /// Path to write the server's PID file. Used by operators to send signals
    /// (e.g. `kill -HUP $(cat /tmp/mini-redis.pid)` for hot reload).
    /// Set to `None` to disable PID file creation.
    pub pid_file: Option<Located<String>>,
}

impl Default for ServerSpec {
    fn default() -> Self {
        ServerSpec {
            hostname: Located::detached("127.0.0.1".to_string()),
            port: Located::detached(6379),
            max_connections: Located::detached(250),
            shutdown_timeout_secs: Located::detached(30),
            pub_sub_channel_capacity: Located::detached(DEFAULT_PUB_SUB_CHANNEL_CAPACITY as i64),
            pid_file: None,
        }
    }
}

pub const DEFAULT_PUB_SUB_CHANNEL_CAPACITY: usize = 1024;

range_constraint!(PORT, i64, min: 1, max: 65535);
range_constraint!(MAX_CONNECTIONS, i64, min: 1, max: 250, help: "A friendly reminder that this is not a production server ;)");
range_constraint!(SHUTDOWN_TIMEOUT, i64, min: 1, max: 3600, units: "s", help: "Keep this under an hour for responsive shutdowns.");
range_constraint!(PUB_SUB_CAPACITY, i64, min: 1, max: 65536, help: "Large capacities hold more messages for slow subscribers but use more memory.");

impl Validate for ServerSpec {
    fn validate(&self, report: &mut Report) {
        if self.hostname.value.is_empty() {
            report
                .error("hostname cannot be empty")
                .at(self.hostname.span)
                .help("Set hostname to a valid DNS name or IP address.")
                .emit();
        } else {
            // Validate the address here so a bad hostname is collected during
            // spec validation rather than only surfacing at lowering, which the
            // error gate skips. The parsed value is discarded; lowering parses
            // it again through the same shared validator.
            net::to_socket_addr(&self.hostname, &self.port, report);
        }

        PORT.check_located(&self.port, "port", report);
        MAX_CONNECTIONS.check_located(&self.max_connections, "max_connections", report);
        SHUTDOWN_TIMEOUT.check_located(
            &self.shutdown_timeout_secs,
            "shutdown_timeout_secs",
            report,
        );
        PUB_SUB_CAPACITY.check_located(
            &self.pub_sub_channel_capacity,
            "pub_sub_channel_capacity",
            report,
        );

        if let Some(pid_file) = &self.pid_file {
            path::parent_exists(pid_file, "pid_file", report);
        }
    }
}
