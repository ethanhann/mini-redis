use confval::{
    range_constraint, validate_range_field, RangeConstraint, SimpleOrigin, ValidateSpec,
    ValidationReport,
};

use super::server_issues;
use super::ServerSpec;

range_constraint!(PORT_RANGE, u16, min: 1, max: 65535);
range_constraint!(MAX_CONNECTIONS_RANGE, usize, min: 1, max: 250, help: "A friendly reminder that this is not a production server ;)");
range_constraint!(SHUTDOWN_TIMEOUT_RANGE, u64, min: 1, max: 300, units: "s", help: "Keep this under 5 minutes for responsive shutdowns.");

impl ValidateSpec<SimpleOrigin> for ServerSpec {
    fn validate(&self, origin: &SimpleOrigin, report: &mut ValidationReport<SimpleOrigin>) {
        validate_range_field!(PORT_RANGE, self.port, report, origin);
        validate_range_field!(MAX_CONNECTIONS_RANGE, self.max_connections, report, origin);
        validate_range_field!(SHUTDOWN_TIMEOUT_RANGE, self.shutdown_timeout_secs, report, origin);

        if self.hostname.is_empty() {
            report.push(server_issues::hostname_empty(origin));
        }
    }
}
