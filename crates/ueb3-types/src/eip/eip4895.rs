//! Extra types defined in [EIP-4895].
//!
//! [EIP-4895]: https://eips.ethereum.org/EIPS/eip-4895

use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;

// -----------------------------------------------------------------------------
// Validator Withdrawal
// -----------------------------------------------------------------------------

/// A validator withdrawal that has been validated at the consensus layer.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Withdrawal {
  /// A monotonically increasing counter that uniquely identifies the withdrawal.
  #[serde(with = "ueb3_serde::quantity")]
  pub index: u64,
  /// The consensus layer index of the validator that the withdrawal corresponds to.
  #[serde(rename = "validatorIndex", with = "ueb3_serde::quantity")]
  pub validator: u64,
  /// The recipient for the withdrawn ETH.
  pub address: Address,
  /// A nonzero amount of ETH represented in Gwei.
  #[serde(with = "ueb3_serde::quantity")]
  pub amount: u64,
}

// -----------------------------------------------------------------------------
// Validator Withdrawal List
// -----------------------------------------------------------------------------

define_list! {
  #[copy]
  WithdrawalList = [Withdrawal]
}
