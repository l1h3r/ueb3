use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::H256;

// -----------------------------------------------------------------------------
// Contract Creator
// -----------------------------------------------------------------------------

/// Information about an address that initiated the creation of a contract.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractCreator {
  /// The hash of the transaction who created the contract.
  pub hash: H256,
  /// The address who directly created the contract.
  pub creator: Address,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::ots::ContractCreator;

  #[test]
  fn test_serde() {
    const HASH_A: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
    const HASH_B: &str = "0xf49f90aa5a268c40001d1227b76bb4dd8247f18361fcad9fffd4a7a44f1320d3";

    const CREATOR_A: &str = "0x0000000000000000000000000000000000000000";
    const CREATOR_B: &str = "0x4e59b44847b379578588920ca78fbf26c0b4956c";

    let a: ContractCreator = ContractCreator::default();

    let b: ContractCreator = ContractCreator {
      hash: HASH_B.parse().unwrap(),
      creator: CREATOR_B.parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "ContractCreator",
        len: 2,
      },
      Token::Str("hash"),
      Token::Str(HASH_A),
      Token::Str("creator"),
      Token::Str(CREATOR_A),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "ContractCreator",
        len: 2,
      },
      Token::Str("hash"),
      Token::Str(HASH_B),
      Token::Str("creator"),
      Token::Str(CREATOR_B),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
