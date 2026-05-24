use super::client_config::ClientConfig;
use crate::conf::types::specification::client::ClientSpec;

impl From<ClientSpec> for ClientConfig {
    fn from(spec: ClientSpec) -> Self {
        ClientConfig {
            read_buffer_bytes: spec.read_buffer_bytes as usize,
            pub_sub_channel_capacity: spec.pub_sub_channel_capacity as usize,
        }
    }
}
