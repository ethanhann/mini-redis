use crate::Error;
use serde::Deserialize;
use std::path::Path;

use super::types::{ClientSpec, ServerSpec};

#[derive(Default, Deserialize)]
#[serde(default)]
pub(super) struct ConfigFile {
    pub server: ServerSpec,
    pub client: ClientSpec,
}

pub(super) fn parse_config_file(path: &Path) -> Result<ConfigFile, Error> {
    let contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}
