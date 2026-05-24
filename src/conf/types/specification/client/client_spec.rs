use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct ClientSpec {
    /// Read buffer size per connection. Real applications will want to tune this
    /// value to their specific use case. There is a high likelihood that a larger
    /// read buffer will work better.
    pub read_buffer_bytes: u32,

    /// Capacity of each pub/sub broadcast channel. A message is stored in the
    /// channel until all subscribers have seen it, so a slow subscriber could
    /// result in messages being held indefinitely. When the channel's capacity
    /// fills up, publishing will result in old messages being dropped. This
    /// prevents slow consumers from blocking the entire system.
    pub pub_sub_channel_capacity: u32,
}

impl Default for ClientSpec {
    fn default() -> Self {
        ClientSpec {
            read_buffer_bytes: 4096,
            pub_sub_channel_capacity: 1024,
        }
    }
}
