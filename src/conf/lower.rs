use crate::Error;
use std::convert::TryFrom;

use super::loader::ResolvedConfig;
use super::types::{ClientConfig, ClientSpec, ServerConfig, ServerSpec};

pub(super) fn lower_config(
    server_spec: ServerSpec,
    client_spec: ClientSpec,
) -> Result<ResolvedConfig, Error> {
    let server = ServerConfig::try_from(server_spec)?;
    let client = ClientConfig::from(client_spec);
    Ok(ResolvedConfig { server, client })
}
