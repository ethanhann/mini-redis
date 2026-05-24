use crate::Error;
use serde::Deserialize;
use std::path::PathBuf;

use super::types::specification::client::ClientSpec;
use super::types::specification::server::ServerSpec;

#[derive(Deserialize)]
#[serde(default)]
pub(super) struct ConfigFile {
    pub server: ServerSpec,
    pub client: ClientSpec,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            server: ServerSpec::default(),
            client: ClientSpec::default(),
        }
    }
}

pub(super) fn parse_config_file(path: &PathBuf) -> Result<ConfigFile, Error> {
    let contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}
