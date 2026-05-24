use crate::Error;
use confval::{SimpleOrigin, ValidationReport};
use std::convert::TryFrom;

use super::loader::ResolvedConfig;
use super::types::runtime::client_config::ClientConfig;
use super::types::runtime::server_config::ServerConfig;
use super::types::specification::client::ClientSpec;
use super::types::specification::server::ServerSpec;

pub(super) fn lower_config(
    server_spec: ServerSpec,
    client_spec: ClientSpec,
    report: ValidationReport<SimpleOrigin>,
) -> Result<ResolvedConfig, Error> {
    let has_warnings = report.has_warnings();
    let server = ServerConfig::try_from(server_spec)?;
    let client = ClientConfig::from(client_spec);
    Ok(ResolvedConfig {
        server,
        client,
        report: if has_warnings { Some(report) } else { None },
    })
}
