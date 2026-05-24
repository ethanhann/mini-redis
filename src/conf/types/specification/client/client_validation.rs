use confval::{
    range_constraint, validate_range_field, RangeConstraint, SimpleOrigin, ValidateSpec,
    ValidationReport,
};

use super::ClientSpec;

range_constraint!(READ_BUFFER_RANGE, usize, min: 1024, max: 65536, units: " bytes", help: "Buffer sizes below 1KB hurt throughput; above 64KB waste memory per connection.");
range_constraint!(PUB_SUB_CAPACITY_RANGE, usize, min: 1, max: 65536, help: "Large capacities hold more messages for slow subscribers but use more memory.");

impl ValidateSpec<SimpleOrigin> for ClientSpec {
    fn validate(&self, origin: &SimpleOrigin, report: &mut ValidationReport<SimpleOrigin>) {
        validate_range_field!(READ_BUFFER_RANGE, self.read_buffer_bytes, report, origin);
        validate_range_field!(PUB_SUB_CAPACITY_RANGE, self.pub_sub_channel_capacity, report, origin);
    }
}
