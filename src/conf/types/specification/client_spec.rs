use confval::diagnostic::Report;
use confval::pipeline::Validate;
use confval::source::Located;
use confval::{range_constraint, RangeConstraint};

pub const DEFAULT_READ_BUFFER_BYTES: usize = 4096;
pub const DEFAULT_PUB_SUB_CHANNEL_CAPACITY: usize = 1024;

#[derive(confval::Spec)]
pub struct ClientSpec {
    /// Read buffer size per connection. Real applications will want to tune this
    /// value to their specific use case. There is a high likelihood that a larger
    /// read buffer will work better.
    #[confval(default = 4096)]
    pub read_buffer_bytes: Located<i64>,

    /// Capacity of each pub/sub broadcast channel. A message is stored in the
    /// channel until all subscribers have seen it, so a slow subscriber could
    /// result in messages being held indefinitely. When the channel's capacity
    /// fills up, publishing will result in old messages being dropped. This
    /// prevents slow consumers from blocking the entire system.
    #[confval(default = 1024)]
    pub pub_sub_channel_capacity: Located<i64>,
}

impl Default for ClientSpec {
    fn default() -> Self {
        ClientSpec {
            read_buffer_bytes: Located::detached(DEFAULT_READ_BUFFER_BYTES as i64),
            pub_sub_channel_capacity: Located::detached(DEFAULT_PUB_SUB_CHANNEL_CAPACITY as i64),
        }
    }
}

range_constraint!(READ_BUFFER, i64, min: 1024, max: 65536, units: " bytes", help: "Buffer sizes below 1KB hurt throughput; above 64KB waste memory per connection.");
range_constraint!(PUB_SUB_CAPACITY, i64, min: 1, max: 65536, help: "Large capacities hold more messages for slow subscribers but use more memory.");

impl Validate for ClientSpec {
    fn validate(&self, report: &mut Report) {
        READ_BUFFER.check_located(&self.read_buffer_bytes, "read_buffer_bytes", report);
        PUB_SUB_CAPACITY.check_located(
            &self.pub_sub_channel_capacity,
            "pub_sub_channel_capacity",
            report,
        );
    }
}
