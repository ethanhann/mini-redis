#[derive(PartialEq)]
pub struct ClientConfig {
    pub read_buffer_bytes: usize,
    pub pub_sub_channel_capacity: usize,
}
