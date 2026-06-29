use std::net::SocketAddr;

use confval::diagnostic::Report;
use confval::source::Located;

/// Parse `hostname:port` into a [`SocketAddr`], reporting at the hostname span
/// on failure.
///
/// Shared by spec validation (which discards the value) and config lowering
/// (which keeps it), so the address is checked even when other errors gate
/// lowering off.
pub fn to_socket_addr(
    hostname: &Located<String>,
    port: &Located<i64>,
    report: &mut Report,
) -> Option<SocketAddr> {
    match format!("{}:{}", hostname.value, port.value).parse::<SocketAddr>() {
        Ok(addr) => Some(addr),
        Err(_) => {
            report
                .error(format!("invalid address: {}:{}", hostname.value, port.value))
                .at(hostname.span)
                .help("Set hostname to an IP address, e.g. \"127.0.0.1\".")
                .emit();
            None
        }
    }
}
