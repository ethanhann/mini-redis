use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct ServerSpec {
    pub hostname: String,
    pub port: u16,

    /// Maximum number of concurrent connections the redis server will accept.
    ///
    /// When this limit is reached, the server will stop accepting connections until
    /// an active connection terminates.
    ///
    /// A real application will want to make this value configurable, but for this
    /// example, it is hard coded.
    ///
    /// This is also set to a pretty low value to discourage using this in
    /// production (you'd think that all the disclaimers would make it obvious that
    /// this is not a serious project... but I thought that about mini-http as
    /// well).
    pub max_connections: usize,

    pub shutdown_timeout_secs: u64,
}

impl Default for ServerSpec {
    fn default() -> Self {
        ServerSpec {
            hostname: "127.0.0.1".to_string(),
            port: 6379,
            max_connections: 250,
            shutdown_timeout_secs: 30,
        }
    }
}
