use ueb3_rpc::Result;
use ueb3_types::eth::Address;
use ueb3_types::eth::Enode;
use ueb3_types::eth::U64;

use crate::provider::Provider;
use crate::temporary::Unimplemented;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsNet for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (net)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `net` namespace.
pub trait NsNet: private::Sealed + Provider {
  /// Returns an indication if the node is listening for network connections.
  #[doc(alias = "net_listening")]
  #[inline]
  fn listening(&self) -> Result<bool> {
    self.client().rpc("net_listening", ())
  }

  /// Returns the number of connected peers.
  #[doc(alias = "net_peerCount")]
  #[inline]
  fn peer_count(&self) -> Result<u64> {
    self.client().rpc("net_peerCount", ()).map(U64::into_u64)
  }

  /// Returns the devp2p network ID (e.g. 1 for mainnet, 5 for goerli).
  #[doc(alias = "net_version")]
  #[inline]
  fn version(&self) -> Result<String> {
    self.client().rpc("net_version", ())
  }

  // ---------------------------------------------------------------------------
  // Besu Extensions
  // ---------------------------------------------------------------------------

  /// Returns the enode URL of the local node.
  #[doc(alias = "net_enode")]
  #[inline]
  fn enode(&self) -> Result<Enode> {
    self.client().rpc("net_enode", ())
  }

  /// Returns enabled services and socket addresses for each service.
  #[doc(alias = "net_services")]
  #[inline]
  fn services(&self) -> Result<Unimplemented> {
    self.client().rpc("net_services", ())
  }

  // ---------------------------------------------------------------------------
  // Nethermind Extensions
  // ---------------------------------------------------------------------------

  /// Returns the address of the local node.
  #[doc(alias = "net_localAddress")]
  #[inline]
  fn local_address(&self) -> Result<Address> {
    self.client().rpc("net_localAddress", ())
  }

  /// Returns the enode URL of the local node.
  #[doc(alias = "net_localEnode")]
  #[inline]
  fn local_enode(&self) -> Result<Enode> {
    self.client().rpc("net_localEnode", ())
  }
}
