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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip7702::Authorization;
  use crate::eip::eip7702::AuthorizationList;
  use crate::eth::U256;
  use crate::eth::YParity;

  #[test]
  fn test_serde() {
    const R: &str = "0x28da8576c6e73b424e8cf57e46c683fae141ecbb7150e8ede3b673e82a6f90a6";
    const S: &str = "0x1ca5eb17d037909806a3c5cca595633369cdcb2f59b3916171e85782c01a296d";

    let a: AuthorizationList = AuthorizationList::from([Authorization {
      chain_id: U256::from(0x1),
      address: Default::default(),
      nonce: 0x3,
      y_parity: YParity::Odd,
      r: R.parse().unwrap(),
      s: S.parse().unwrap(),
    }]);

    let tokens_a: &[Token] = &[
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "Authorization",
        len: 6,
      },
      Token::Str("chainId"),
      Token::Str("0x1"),
      Token::Str("address"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("nonce"),
      Token::Str("0x3"),
      Token::Str("yParity"),
      Token::UnitVariant {
        name: "YParity",
        variant: "0x1",
      },
      Token::Str("r"),
      Token::Str(R),
      Token::Str("s"),
      Token::Str(S),
      Token::StructEnd,
      Token::SeqEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
  }
}
