//! Reusable network checks the server validators and the lowering step call.

use confval::prelude::{Located, Report};
use std::net::IpAddr;

/// Reports a value that is not an IPv4 or an IPv6 literal.
pub fn check_ip_addr(value: &Located<String>, field: &'static str, report: &mut Report) {
    if value.value.parse::<IpAddr>().is_err() {
        report
            .error(format!("{field} must be an IP address"))
            .at(value.span)
            .help("Write an IPv4 or an IPv6 literal, such as 127.0.0.1 or ::1")
            .emit();
    }
}

/// Narrows a checked address to an `IpAddr` at the lowering boundary.
///
/// A failure here means a validation rule is absent rather than that the
/// operator made a mistake, so the message says so.
pub fn narrow_ip_addr(value: &Located<String>, report: &mut Report) -> Option<IpAddr> {
    match value.value.parse::<IpAddr>() {
        Ok(address) => Some(address),
        Err(_) => {
            report
                .error("bind is not an IP address")
                .at(value.span)
                .help("This value reached lowering unchecked. Add a validation rule for it.")
                .emit();
            None
        }
    }
}
