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
use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::H256;
use crate::eth::U64;
use crate::eth::U256;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Ethereum Transaction Payload (eip-4844)
// -----------------------------------------------------------------------------

/// A transaction object (eip-4844).
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionEip4844 {
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
  /// The maximum total fee per gas the sender is willing to pay for blob gas.
  pub max_fee_per_blob_gas: Wei,
  /// The amount of ETH transferred in the transaction.
  pub value: Wei,
  /// Additional data included with the transaction.
  pub input: Bytes,
  /// A list of addresses/storage keys that the transaction plans to access.
  pub access_list: AccessList,
  /// A list of versioned blob hashes associated with the transaction's data blobs.
  pub blob_versioned_hashes: Vec<H256>,
}

impl TransactionEip4844 {
  /// The [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  pub const TYPE: TransactionType = TransactionType::Eip4844;

  pub(crate) const FIELDS: &[&str] = &[
    "chainId",
    "from",
    "to",
    "nonce",
    "gas",
    "gasPrice",
    "maxPriorityFeePerGas",
    "maxFeePerGas",
    "maxFeePerBlobGas",
    "value",
    "input",
    "accessList",
    "blobVersionedHashes",
  ];
}

impl Debug for TransactionEip4844 {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionEip4844")
      .field("chain_id", &self.chain_id)
      .field("from", &self.from)
      .field("to", &self.to.format())
      .field("nonce", &self.nonce)
      .field("gas_limit", &self.gas_limit)
      .field("gas_price", &self.gas_price.format())
      .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
      .field("max_fee_per_gas", &self.max_fee_per_gas)
      .field("max_fee_per_blob_gas", &self.max_fee_per_blob_gas)
      .field("value", &self.value)
      .field("input", &self.input)
      .field("access_list", &self.access_list)
      .field("blob_versioned_hashes", &self.blob_versioned_hashes)
      .finish()
  }
}

impl SerializeFlatten for TransactionEip4844 {
  const NAME: &'static str = "TransactionEip4844";

  #[inline]
  fn fields(&self) -> usize {
    12 + usize::from(self.gas_price.is_some())
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
    state.serialize_field("maxFeePerBlobGas", &self.max_fee_per_blob_gas)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;
    state.serialize_field("accessList", &self.access_list)?;
    state.serialize_field("blobVersionedHashes", &self.blob_versioned_hashes)?;

    Ok(())
  }
}

impl Serialize for TransactionEip4844 {
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

  use crate::rpc::TransactionEip4844;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x40c35d4faf69234986cb599890c2d2ef546074a9";
    const RECV: &str = "0x0000000000000000000000000000000000000000";
    const BLOB: &str = "0x0175f564b393e44640ecffddce4010a42c1c966413987d8a59a253ef64a0c5cf";

    let input: TransactionEip4844 = TransactionEip4844 {
      chain_id: 0x1,
      from: SEND.parse().unwrap(),
      to: Some(RECV.parse().unwrap()),
      nonce: 0x0,
      gas_limit: "0x5208".parse().unwrap(),
      gas_price: Some("0xd364c1438".parse().unwrap()),
      max_priority_fee_per_gas: "0x3b9aca00".parse().unwrap(),
      max_fee_per_gas: "0x1d1e4e4ea00".parse().unwrap(),
      max_fee_per_blob_gas: "0x3b9aca00".parse().unwrap(),
      value: Default::default(),
      input: Default::default(),
      access_list: Default::default(),
      blob_versioned_hashes: vec![BLOB.parse().unwrap()],
    };

    let tokens: &[Token] = &[
      Token::Struct {
        name: "TransactionEip4844",
        len: 13,
      },
      Token::Str("chainId"),
      Token::Str("0x1"),
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
      Token::Some,
      Token::Str("0xd364c1438"),
      Token::Str("maxPriorityFeePerGas"),
      Token::Str("0x3b9aca00"),
      Token::Str("maxFeePerGas"),
      Token::Str("0x1d1e4e4ea00"),
      Token::Str("maxFeePerBlobGas"),
      Token::Str("0x3b9aca00"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("accessList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("blobVersionedHashes"),
      Token::Seq { len: Some(1) },
      Token::Str(BLOB),
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&input.readable(), tokens);
  }
}
