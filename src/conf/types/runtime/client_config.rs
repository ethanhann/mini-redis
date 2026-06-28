use confval::pipeline::narrow;

use crate::conf::types::specification::ClientSpec;

#[derive(Debug, PartialEq, confval::Config)]
#[confval(lower_from = ClientSpec, validate)]
pub struct ClientConfig {
    #[confval(lower(from = read_buffer_bytes, with = narrow::i64_to_usize))]
    pub read_buffer_bytes: usize,

    #[confval(lower(from = pub_sub_channel_capacity, with = narrow::i64_to_usize))]
    pub pub_sub_channel_capacity: usize,
}
