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
use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::U64;
use crate::eth::U256;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Ethereum Transaction Payload (legacy)
// -----------------------------------------------------------------------------

/// A transaction object (legacy).
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionLegacy {
  /// A unique identifier for which chain the transaction occured on.
  ///
  /// Only specified for transactions included after [EIP-155].
  ///
  /// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
  #[serde(default, with = "ueb3_serde::quantity::opt")]
  pub chain_id: Option<u64>,
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
  /// The gas price willing to be paid by the sender.
  pub gas_price: Wei,
  /// The amount of ETH transferred in the transaction.
  pub value: Wei,
  /// Additional data included with the transaction.
  pub input: Bytes,
}

impl TransactionLegacy {
  /// The [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  pub const TYPE: TransactionType = TransactionType::Legacy;

  pub(crate) const FIELDS: &[&str] = &[
    "chainId",
    "from",
    "to",
    "nonce",
    "gas",
    "gasPrice",
    "value",
    "input",
  ];
}

impl Debug for TransactionLegacy {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionLegacy")
      .field("chain_id", &self.chain_id.format())
      .field("from", &self.from)
      .field("to", &self.to.format())
      .field("nonce", &self.nonce)
      .field("gas_limit", &self.gas_limit)
      .field("gas_price", &self.gas_price)
      .field("value", &self.value)
      .field("input", &self.input)
      .finish()
  }
}

impl SerializeFlatten for TransactionLegacy {
  const NAME: &'static str = "TransactionLegacy";

  #[inline]
  fn fields(&self) -> usize {
    7 + usize::from(self.chain_id.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    if let Some(value) = self.chain_id {
      state.serialize_field("chainId", &U64::from_u64(value))?;
    } else {
      state.skip_field("chainId")?;
    }

    state.serialize_field("from", &self.from)?;
    state.serialize_field("to", &self.to)?;
    state.serialize_field("nonce", &U64::from_u64(self.nonce))?;
    state.serialize_field("gas", &self.gas_limit)?;
    state.serialize_field("gasPrice", &self.gas_price)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;

    Ok(())
  }
}

impl Serialize for TransactionLegacy {
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

  use crate::rpc::TransactionLegacy;

  #[test]
  fn test_serde() {
    const SEND: &str = "0xa1e4380a3b1f749673e270229993ee55f35663b4";
    const RECV: &str = "0x5df9b87991262f6ba471f09758cde1c0fc1de734";

    let input: TransactionLegacy = TransactionLegacy {
      chain_id: None,
      from: SEND.parse().unwrap(),
      to: Some(RECV.parse().unwrap()),
      nonce: 0x0,
      gas_limit: "0x5208".parse().unwrap(),
      gas_price: "0x2d79883d2000".parse().unwrap(),
      value: "0x7a69".parse().unwrap(),
      input: Default::default(),
    };

    let tokens: &[Token] = &[
      Token::Struct {
        name: "TransactionLegacy",
        len: 7,
      },
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("to"),
      Token::Some,
      Token::Str(RECV),
      Token::Str("nonce"),
      Token::Str("0x0"),
      Token::Str("gas"),
      Token::Str("0x5208"),
      Token::Str("gasPrice"),
      Token::Str("0x2d79883d2000"),
      Token::Str("value"),
      Token::Str("0x7a69"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::StructEnd,
    ];

    assert_tokens(&input.readable(), tokens);
  }
}
