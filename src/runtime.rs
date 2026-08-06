use tokio::time::Duration;

use crate::conf::ServerConfig;

#[derive(Debug)]
pub(crate) struct RuntimeConfig {
    pub pub_sub_channel_capacity: usize,
    pub shutdown_timeout: Duration,
}

impl RuntimeConfig {
    pub fn new(server: &ServerConfig) -> Self {
        RuntimeConfig {
            pub_sub_channel_capacity: server.pub_sub_channel_capacity,
            shutdown_timeout: server.shutdown_timeout,
        }
    }
}
