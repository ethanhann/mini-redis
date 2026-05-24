use super::types::runtime::client_config::ClientConfig;
use super::types::runtime::server_config::ServerConfig;

pub enum ConfigChange {
    NoChange,
    RuntimeOnly,
    ListenerChanged,
}

pub fn classify_config_change(
    old_server: &ServerConfig,
    old_client: &ClientConfig,
    new_server: &ServerConfig,
    new_client: &ClientConfig,
) -> ConfigChange {
    if old_server == new_server && old_client == new_client {
        return ConfigChange::NoChange;
    }

    if old_server.listener_identity() != new_server.listener_identity() {
        ConfigChange::ListenerChanged
    } else {
        ConfigChange::RuntimeOnly
    }
}
