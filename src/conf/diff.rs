use super::types::ServerConfig;

pub enum ConfigChange {
    NoChange,
    RuntimeOnly,
    ListenerChanged,
}

pub fn classify_config_change(
    old_server: &ServerConfig,
    new_server: &ServerConfig,
) -> ConfigChange {
    if old_server == new_server {
        return ConfigChange::NoChange;
    }

    if old_server.listener_identity() != new_server.listener_identity() {
        ConfigChange::ListenerChanged
    } else {
        ConfigChange::RuntimeOnly
    }
}
