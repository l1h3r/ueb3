use ueb3_rpc::Result;
use ueb3_types::eth::Address;
use ueb3_types::rpc::txpool;

use crate::provider::Provider;
use crate::temporary::Unimplemented;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsTxPool for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (txpool)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `txpool` namespace.
pub trait NsTxPool: private::Sealed + Provider {
  /// Returns the exact details of all the transactions currently pending for
  /// inclusion in the next block(s), as well as the ones that are being
  /// scheduled for future execution only.
  #[doc(alias = "txpool_content")]
  #[inline]
  fn content(&self) -> Result<txpool::Content> {
    self.client().rpc("txpool_content", ())
  }

  /// Returns the exact details of all the transactions currently pending for
  /// inclusion in the next block(s), as well as the ones that are being
  /// scheduled for future execution only.
  #[doc(alias = "txpool_contentFrom")]
  #[inline]
  fn content_from(&self, address: &Address) -> Result<txpool::ContentFrom> {
    self.client().rpc("txpool_contentFrom", (address,))
  }

  /// Returns a textual summary of all the transactions currently pending for
  /// inclusion in the next block(s), as well as the ones that are being
  /// scheduled for future execution only.
  #[doc(alias = "txpool_inspect")]
  #[inline]
  fn inspect(&self) -> Result<txpool::Inspect> {
    self.client().rpc("txpool_inspect", ())
  }

  /// Returns the number of transactions currently pending for inclusion in the
  /// next block(s), as well as the ones that are being scheduled for future
  /// execution only.
  #[doc(alias = "txpool_status")]
  #[inline]
  fn status(&self) -> Result<txpool::Status> {
    self.client().rpc("txpool_status", ())
  }

  // ---------------------------------------------------------------------------
  // Besu Extensions
  // ---------------------------------------------------------------------------

  /// Returns a list of pending transactions that match the given filter.
  #[doc(alias = "txpool_besuPendingTransactions")]
  #[inline]
  fn besu_pending(&self, count: u64, filter: &Unimplemented) -> Result<Unimplemented> {
    self
      .client()
      .rpc("txpool_besuPendingTransactions", (count, filter))
  }

  /// Returns statistics about the node transaction pool.
  #[doc(alias = "txpool_besuStatistics")]
  #[inline]
  fn besu_statistics(&self) -> Result<Vec<Unimplemented>> {
    self.client().rpc("txpool_besuStatistics", ())
  }

  /// Returns a list of transactions in the node transaction pool.
  #[doc(alias = "txpool_besuTransactions")]
  #[inline]
  fn besu_transactions(&self) -> Result<Vec<Unimplemented>> {
    self.client().rpc("txpool_besuTransactions", ())
  }
}
