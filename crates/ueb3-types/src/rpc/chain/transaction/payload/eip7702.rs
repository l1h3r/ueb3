use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten;
use ueb3_utils::traits::OptionExt as _;

use crate::eip::eip2718::TransactionType;
use crate::eip::eip2930::AccessList;
use crate::eip::eip7702::AuthorizationList;
use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::U64;
use crate::eth::U256;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Ethereum Transaction Payload (eip-7702)
// -----------------------------------------------------------------------------

/// A transaction object (eip-7702).
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionEip7702 {
  /// A unique identifier for which chain the transaction occured on.
  #[serde(with = "ueb3_serde::quantity")]
  pub chain_id: u64,
  /// The address of the transaction sender.
  pub from: Address,
  /// The address of the transaction receiver. `None` when contract creation.
  pub to: Option<Address>,
  /// The number of prior transactions made by the sender.
  #[serde(with = "ueb3_serde::quantity")]
  pub nonce: u64,
  /// The total amount of gas provided by the sender.
  #[serde(rename = "gas")]
  pub gas_limit: U256,
  /// The effective gas price paid by the sender.
  ///
  /// For transactions not yet included in a block, this value should be set
  /// equal to the max fee per gas.
  ///
  /// This field is DEPRECATED, please transition to using `effective_gas_price`
  /// in the receipt object going forward.
  pub gas_price: Option<Wei>,
  /// The maximum fee per gas the sender is willing to pay to miners.
  pub max_priority_fee_per_gas: Wei,
  /// The maximum total fee per gas the sender is willing to pay (includes the
  /// network / base fee and miner / priority fee).
  pub max_fee_per_gas: Wei,
  /// The amount of ETH transferred in the transaction.
  pub value: Wei,
  /// Additional data included with the transaction.
  pub input: Bytes,
  /// A list of addresses/storage keys that the transaction plans to access.
  pub access_list: AccessList,
  /// A list of authorizations for the transaction.
  pub authorization_list: AuthorizationList,
}

impl TransactionEip7702 {
  /// The [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  pub const TYPE: TransactionType = TransactionType::Eip7702;

  pub(crate) const FIELDS: &[&str] = &[
    "chainId",
    "from",
    "to",
    "nonce",
    "gas",
    "gasPrice",
    "maxPriorityFeePerGas",
    "maxFeePerGas",
    "value",
    "input",
    "accessList",
    "authorizationList",
  ];
}

impl Debug for TransactionEip7702 {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionEip7702")
      .field("chain_id", &self.chain_id)
      .field("from", &self.from)
      .field("to", &self.to.format())
      .field("nonce", &self.nonce)
      .field("gas_limit", &self.gas_limit)
      .field("gas_price", &self.gas_price.format())
      .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
      .field("max_fee_per_gas", &self.max_fee_per_gas)
      .field("value", &self.value)
      .field("input", &self.input)
      .field("access_list", &self.access_list)
      .field("authorization_list", &self.authorization_list)
      .finish()
  }
}

impl SerializeFlatten for TransactionEip7702 {
  const NAME: &'static str = "TransactionEip7702";

  #[inline]
  fn fields(&self) -> usize {
    11 + usize::from(self.gas_price.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("chainId", &U64::from_u64(self.chain_id))?;
    state.serialize_field("from", &self.from)?;
    state.serialize_field("to", &self.to)?;
    state.serialize_field("nonce", &U64::from_u64(self.nonce))?;
    state.serialize_field("gas", &self.gas_limit)?;

    if self.gas_price.is_some() {
      state.serialize_field("gasPrice", &self.gas_price)?;
    } else {
      state.skip_field("gasPrice")?;
    }

    state.serialize_field("maxPriorityFeePerGas", &self.max_priority_fee_per_gas)?;
    state.serialize_field("maxFeePerGas", &self.max_fee_per_gas)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;
    state.serialize_field("accessList", &self.access_list)?;
    state.serialize_field("authorizationList", &self.authorization_list)?;

    Ok(())
  }
}

impl Serialize for TransactionEip7702 {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.serialize_struct(serializer)
  }
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
  use crate::eth::YParity;
  use crate::rpc::TransactionEip7702;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x86628a07320098baa038848e88a4217037fb898b";
    const RECV: &str = "0x86628a07320098baa038848e88a4217037fb898b";

    const AUTH_R: &str = "0x28da8576c6e73b424e8cf57e46c683fae141ecbb7150e8ede3b673e82a6f90a6";
    const AUTH_S: &str = "0x1ca5eb17d037909806a3c5cca595633369cdcb2f59b3916171e85782c01a296d";

    let input: TransactionEip7702 = TransactionEip7702 {
      chain_id: 0x1,
      from: SEND.parse().unwrap(),
      to: Some(RECV.parse().unwrap()),
      nonce: 0x2,
      gas_limit: "0x11017".parse().unwrap(),
      gas_price: Some("0x2c55791e0".parse().unwrap()),
      max_priority_fee_per_gas: "0x16673a77d".parse().unwrap(),
      max_fee_per_gas: "0x2f2f04d66".parse().unwrap(),
      value: Default::default(),
      input: Default::default(),
      access_list: Default::default(),
      authorization_list: AuthorizationList::from([Authorization {
        chain_id: "0x1".parse().unwrap(),
        address: Default::default(),
        nonce: 0x3,
        y_parity: YParity::Odd,
        r: AUTH_R.parse().unwrap(),
        s: AUTH_S.parse().unwrap(),
      }]),
    };

    let tokens: &[Token] = &[
      Token::Struct {
        name: "TransactionEip7702",
        len: 12,
      },
      Token::Str("chainId"),
      Token::Str("0x1"),
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("to"),
      Token::Some,
      Token::Str(RECV),
      Token::Str("nonce"),
      Token::Str("0x2"),
      Token::Str("gas"),
      Token::Str("0x11017"),
      Token::Str("gasPrice"),
      Token::Some,
      Token::Str("0x2c55791e0"),
      Token::Str("maxPriorityFeePerGas"),
      Token::Str("0x16673a77d"),
      Token::Str("maxFeePerGas"),
      Token::Str("0x2f2f04d66"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("accessList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("authorizationList"),
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
      Token::Str(AUTH_R),
      Token::Str("s"),
      Token::Str(AUTH_S),
      Token::StructEnd,
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&input.readable(), tokens);
  }
}
