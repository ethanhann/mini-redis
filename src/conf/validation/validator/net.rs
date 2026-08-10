//! Address checks that hold no mini-redis rules of their own.
//!
//! A `Validate` impl calls [`check_ip_addr`] to report a malformed address at
//! the operator's span. Lowering calls [`lower_ip_addr`] to produce the parsed
//! value. Both read the same `FromStr`, so an address the check accepts is one
//! lowering can parse.

use confval::prelude::{Located, Report};
use std::net::IpAddr;

/// Reports a value that is not an IPv4 or IPv6 literal.
///
/// A hostname is refused here rather than at lowering, because resolving one
/// is a network call the configuration layer does not make.
pub fn check_ip_addr(value: &Located<String>, field: &str, report: &mut Report) {
    if value.value.parse::<IpAddr>().is_err() {
        report
            .error(format!("{field} is not an IP address"))
            .at(value.span)
            .help(format!(
                "Set {field} to an IPv4 or IPv6 literal, such as 127.0.0.1 or ::1"
            ))
            .emit();
    }
}

/// Parses a checked address for a `#[confval(lower(with = ...))]` attribute.
///
/// Reaching the error arm means [`check_ip_addr`] did not run on this field,
/// so the message says as much rather than blaming the operator.
pub fn lower_ip_addr(value: &Located<String>, report: &mut Report) -> Option<IpAddr> {
    match value.value.parse::<IpAddr>() {
        Ok(address) => Some(address),
        Err(_) => {
            report
                .error(format!("{} is not an IP address", value.value))
                .at(value.span)
                .help("This value reached lowering unchecked, which is a missing validation rule")
                .emit();
            None
        }
    }
}
