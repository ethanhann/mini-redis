use confval::{SimpleOrigin, ValidationIssue};

pub(crate) fn hostname_empty(origin: &SimpleOrigin) -> ValidationIssue<SimpleOrigin> {
    ValidationIssue::error_with_help(
        "hostname cannot be empty",
        origin.clone(),
        "Set hostname to a valid DNS name or IP address.",
    )
}
