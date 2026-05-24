use tokio::time::Duration;

use crate::conf::{ClientConfig, ServerConfig};

#[derive(Debug)]
pub(crate) struct RuntimeConfig {
    pub read_buffer_bytes: usize,
    pub pub_sub_channel_capacity: usize,
    pub shutdown_timeout: Duration,
}

impl RuntimeConfig {
    pub fn new(server: &ServerConfig, client: &ClientConfig) -> Self {
        RuntimeConfig {
            read_buffer_bytes: client.read_buffer_bytes,
            pub_sub_channel_capacity: client.pub_sub_channel_capacity,
            shutdown_timeout: server.shutdown_timeout,
        }
    }
}
