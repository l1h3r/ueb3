use crate::eth::U256;

// -----------------------------------------------------------------------------
// Ethereum Transaction Signer
// -----------------------------------------------------------------------------

/// A description of the creator of a transaction signature.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TransactionSigner {
  /// ECDSA recovery id.
  pub v: u64,
  /// ECDSA signature r.
  pub r: U256,
  /// ECDSA signature s.
  pub s: U256,
}
