use confval::source::Located;

use super::types::ServerSpec;

/// The top-level configuration file: a `[server]` block.
///
/// The block maps to a TOML table, which the confval frontend lowers into a
/// nested block parsed through `ServerSpec`'s derived `FromFields` impl.
#[derive(confval::Spec)]
pub(super) struct ConfigFile {
    #[confval(nested)]
    pub server: Located<ServerSpec>,
}
