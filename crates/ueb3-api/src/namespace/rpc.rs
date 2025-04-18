use std::collections::HashMap;
use ueb3_rpc::Result;

use crate::provider::Provider;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsRpc for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (rpc)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `rpc` namespace.
pub trait NsRpc: private::Sealed + Provider {
  /// Returns a map of available JSON-RPC modules and versions.
  #[doc(alias = "rpc_modules")]
  #[inline]
  fn modules(&self) -> Result<HashMap<String, String>> {
    self.client().rpc("rpc_modules", ())
  }
}
