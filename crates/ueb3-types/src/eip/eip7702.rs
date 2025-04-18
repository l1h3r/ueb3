//! Extra types defined in [EIP-7702].
//!
//! [EIP-7702]: https://eips.ethereum.org/EIPS/eip-7702

use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::U256;
use crate::eth::YParity;

// -----------------------------------------------------------------------------
// Authorization List Item
// -----------------------------------------------------------------------------

/// A signed authorization object.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Authorization {
  /// The authorization chain identifier.
  pub chain_id: U256,
  /// The authorization signer address.
  pub address: Address,
  /// The authorization nonce.
  #[serde(with = "ueb3_serde::quantity")]
  pub nonce: u64,
  /// The parity (0 for even, 1 for odd) of the y-value of the secp256k1 signature.
  pub y_parity: YParity,
  /// ECDSA signature r.
  pub r: U256,
  /// ECDSA signature s.
  pub s: U256,
}

// -----------------------------------------------------------------------------
// Authorization List
// -----------------------------------------------------------------------------

define_list! {
  #[copy]
  AuthorizationList = [Authorization]
}
