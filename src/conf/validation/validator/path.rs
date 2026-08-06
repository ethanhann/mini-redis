use std::path::Path;

use confval::diagnostic::Report;
use confval::source::Located;

/// Report an error if the parent directory of `path`'s value does not exist.
///
/// `field` names the offending field for the message. A value with no parent
/// component, or an empty parent (a bare filename), is treated as valid.
pub fn parent_exists(path: &Located<String>, field: &str, report: &mut Report) {
    if let Some(parent) = Path::new(&path.value).parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            report
                .error(format!(
                    "{field} parent directory does not exist: {}",
                    path.value
                ))
                .at(path.span)
                .help("Create the parent directory or choose a path under an existing directory.")
                .emit();
        }
    }
}
