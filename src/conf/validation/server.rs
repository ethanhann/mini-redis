//! The rules an attribute cannot express.
//!
//! A range and a keyword set are recorded on the field itself, so `validate`
//! holds only the format check and the cross-field rule.

use confval::prelude::{Report, Validate};

use crate::conf::types::specification::server::{BackoffSpec, ServerSpec};
use crate::conf::validation::validator::net;

impl Validate for ServerSpec {
    fn validate(&self, report: &mut Report) {
        net::check_ip_addr(&self.bind, "bind", report);
    }
}

impl Validate for BackoffSpec {
    fn validate(&self, report: &mut Report) {
        if self.initial_seconds.value > self.max_seconds.value {
            report
                .error("initial_seconds must not be greater than max_seconds")
                .at(self.initial_seconds.span)
                .related(self.max_seconds.span, "max_seconds declared here")
                .help("Lower initial_seconds, or raise max_seconds.")
                .emit();
        }
    }
}
