//! What the server's settings are allowed to mean.
//!
//! Each impl covers the fields of one spec type and nothing below it.
//! `validate_all` descends into the nested blocks, so `ServerSpec` never calls
//! the limits validator by hand. No rule returns early, so one run reports
//! every problem in the file rather than the first.

use crate::conf::types::runtime::server::LogLevel;
use crate::conf::types::specification::server::{LimitsSpec, ServerSpec};
use crate::conf::validation::validator::net;
use confval::prelude::{range_constraint, RangeConstraint, Report, Validate};

range_constraint!(PORT, i64, min: 1, max: 65535);
range_constraint!(MAX_CONNECTIONS, i64, min: 1, max: 100_000);
range_constraint!(MAX_BACKOFF_SECONDS, i64, min: 1, max: 3600, units: "seconds");

impl Validate for ServerSpec {
    fn validate(&self, report: &mut Report) {
        net::check_ip_addr(&self.bind_address, "bind_address", report);
        PORT.check_located(&self.port, "port", report);
        LogLevel::keyword_set().check_located(&self.log_level, "log_level", report);

        if (1..1024).contains(&self.port.value) {
            report
                .warning(format!(
                    "port {} is privileged and needs elevated permission to bind",
                    self.port.value
                ))
                .at(self.port.span)
                .help("Set port to 1024 or above to run as an unprivileged user")
                .emit();
        }
    }
}

impl Validate for LimitsSpec {
    fn validate(&self, report: &mut Report) {
        MAX_CONNECTIONS.check_located(&self.max_connections, "max_connections", report);
        MAX_BACKOFF_SECONDS.check_located(&self.max_backoff_seconds, "max_backoff_seconds", report);
    }
}
