use mini_redis::conf::{ClientConfig, ServerConfig};
use mini_redis::{
    clients::{BufferedClient, Client},
    server,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

/// A basic "hello world" style test. A server instance is started in a
/// background task. A client instance is then established and used to initialize
/// the buffer. Set and get commands are sent to the server. The response is
/// then evaluated.
#[tokio::test]
async fn pool_key_value_get_set() {
    let (addr, _) = start_server().await;

    let client = Client::connect(addr).await.unwrap();
    let mut client = BufferedClient::buffer(client);

    client.set("hello", "world".into()).await.unwrap();

    let value = client.get("hello").await.unwrap().unwrap();
    assert_eq!(b"world", &value[..])
}

async fn start_server() -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_config = ServerConfig {
        addr,
        max_connections: 250,
        shutdown_timeout: std::time::Duration::from_secs(30),
        pid_file: None,
    };
    let client_config = ClientConfig {
        read_buffer_bytes: 4096,
        pub_sub_channel_capacity: 1024,
    };

    let handle = tokio::spawn(async move {
        server::run(listener, tokio::signal::ctrl_c(), server_config, client_config, None).await
    });

    (addr, handle)
}
