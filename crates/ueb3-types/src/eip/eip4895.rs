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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip4895::Withdrawal;
  use crate::eip::eip4895::WithdrawalList;

  #[test]
  fn test_serde() {
    const ADDRESS: &str = "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f";

    let a: WithdrawalList = WithdrawalList::new();

    let b: WithdrawalList = WithdrawalList::from([Withdrawal {
      index: 0x0,
      validator: 0x9d8c0,
      address: ADDRESS.parse().unwrap(),
      amount: 0x11a33e3760,
    }]);

    let tokens_a: &[Token] = &[Token::Seq { len: Some(0) }, Token::SeqEnd];

    let tokens_b: &[Token] = &[
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "Withdrawal",
        len: 4,
      },
      Token::Str("index"),
      Token::Str("0x0"),
      Token::Str("validatorIndex"),
      Token::Str("0x9d8c0"),
      Token::Str("address"),
      Token::Str(ADDRESS),
      Token::Str("amount"),
      Token::Str("0x11a33e3760"),
      Token::StructEnd,
      Token::SeqEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
