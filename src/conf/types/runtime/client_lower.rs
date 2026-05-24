use super::ClientConfig;
use crate::conf::types::specification::ClientSpec;

impl From<ClientSpec> for ClientConfig {
    fn from(spec: ClientSpec) -> Self {
        ClientConfig {
            read_buffer_bytes: spec.read_buffer_bytes,
            pub_sub_channel_capacity: spec.pub_sub_channel_capacity,
        }
    }
}
