use serde::Deserialize;

pub const DEFAULT_READ_BUFFER_BYTES: usize = 4096;
pub const DEFAULT_PUB_SUB_CHANNEL_CAPACITY: usize = 1024;

#[derive(Deserialize)]
#[serde(default)]
pub struct ClientSpec {
    /// Read buffer size per connection. Real applications will want to tune this
    /// value to their specific use case. There is a high likelihood that a larger
    /// read buffer will work better.
    pub read_buffer_bytes: usize,

    /// Capacity of each pub/sub broadcast channel. A message is stored in the
    /// channel until all subscribers have seen it, so a slow subscriber could
    /// result in messages being held indefinitely. When the channel's capacity
    /// fills up, publishing will result in old messages being dropped. This
    /// prevents slow consumers from blocking the entire system.
    pub pub_sub_channel_capacity: usize,
}

impl Default for ClientSpec {
    fn default() -> Self {
        ClientSpec {
            read_buffer_bytes: DEFAULT_READ_BUFFER_BYTES,
            pub_sub_channel_capacity: DEFAULT_PUB_SUB_CHANNEL_CAPACITY,
        }
    }
}
