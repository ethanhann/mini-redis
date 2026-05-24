use confval::{SimpleOrigin, ValidationIssue};

pub(crate) fn hostname_empty(origin: &SimpleOrigin) -> ValidationIssue<SimpleOrigin> {
    ValidationIssue::error_with_help(
        "hostname cannot be empty",
        origin.clone(),
        "Set hostname to a valid DNS name or IP address.",
    )
}

pub(crate) fn pid_file_parent_missing(
    path: &str,
    origin: &SimpleOrigin,
) -> ValidationIssue<SimpleOrigin> {
    ValidationIssue::error_with_help(
        format!("pid_file parent directory does not exist: {}", path),
        origin.clone(),
        "Create the parent directory or choose a path under an existing directory.",
    )
}
