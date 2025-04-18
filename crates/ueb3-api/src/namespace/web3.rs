use ueb3_rpc::Result;
use ueb3_types::eth::H256;
use ueb3_types::eth::Slice;

use crate::provider::Provider;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsWeb3 for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (web3)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `web3` namespace.
pub trait NsWeb3: private::Sealed + Provider {
  /// Returns the current client version.
  #[doc(alias = "web3_clientVersion")]
  #[inline]
  fn client_version(&self) -> Result<String> {
    self.client().rpc("web3_clientVersion", ())
  }

  /// Returns the `Keccak-256` hash of the given data.
  ///
  /// Note: This is not the standardized `SHA3-256` hash.
  #[doc(alias = "web3_sha3")]
  #[inline]
  fn sha3(&self, data: &Slice) -> Result<H256> {
    self.client().rpc("web3_sha3", (data,))
  }
}
