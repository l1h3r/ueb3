use std::collections::HashMap;
use ueb3_rpc::Result;
use ueb3_types::eip::eip1898::BlockId;
use ueb3_types::eth::Address;
use ueb3_types::eth::Wei;

use crate::provider::Provider;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsReth for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (reth)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `reth` namespace.
pub trait NsReth: private::Sealed + Provider {
  /// Returns all ETH balance changes in a block matching the given block id.
  #[doc(alias = "reth_getBalanceChangesInBlock")]
  #[inline]
  fn balance_changes_in_block(&self, block: BlockId) -> Result<HashMap<Address, Wei>> {
    self.client().rpc("reth_getBalanceChangesInBlock", (block,))
  }
}
