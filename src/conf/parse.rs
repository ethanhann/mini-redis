use confval::source::Located;

use super::types::{ClientSpec, ServerSpec};

/// The top-level configuration file: a `[server]` block and a `[client]` block.
///
/// Both map to TOML tables, which the confval frontend lowers into nested
/// blocks. Each block is parsed through its own derived `FromFields` impl, so a
/// shape error in one does not hide errors in the other.
#[derive(confval::Spec)]
pub(super) struct ConfigFile {
    #[confval(nested)]
    pub server: Located<ServerSpec>,

    #[confval(nested)]
    pub client: Located<ClientSpec>,
}
