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
use crate::eth::U64;
use crate::eth::U256;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Ethereum Transaction Payload (eip-2930)
// -----------------------------------------------------------------------------

/// A transaction object (eip-2930).
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionEip2930 {
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
  /// The gas price willing to be paid by the sender.
  pub gas_price: Wei,
  /// The amount of ETH transferred in the transaction.
  pub value: Wei,
  /// Additional data included with the transaction.
  pub input: Bytes,
  /// A list of addresses/storage keys that the transaction plans to access.
  pub access_list: AccessList,
}

impl TransactionEip2930 {
  /// The [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  pub const TYPE: TransactionType = TransactionType::Eip2930;

  pub(crate) const FIELDS: &[&str] = &[
    "chainId",
    "from",
    "to",
    "nonce",
    "gas",
    "gasPrice",
    "value",
    "input",
    "accessList",
  ];
}

impl Debug for TransactionEip2930 {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionEip2930")
      .field("chain_id", &self.chain_id)
      .field("from", &self.from)
      .field("to", &self.to.format())
      .field("nonce", &self.nonce)
      .field("gas_limit", &self.gas_limit)
      .field("gas_price", &self.gas_price)
      .field("value", &self.value)
      .field("input", &self.input)
      .field("access_list", &self.access_list)
      .finish()
  }
}

impl SerializeFlatten for TransactionEip2930 {
  const NAME: &'static str = "TransactionEip2930";

  #[inline]
  fn fields(&self) -> usize {
    9
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
    state.serialize_field("gasPrice", &self.gas_price)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;
    state.serialize_field("accessList", &self.access_list)?;

    Ok(())
  }
}

impl Serialize for TransactionEip2930 {
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

  use crate::eip::eip2930::Access;
  use crate::eip::eip2930::AccessList;
  use crate::rpc::TransactionEip2930;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x1ced2cef30d40bb3617f8d455071b69f3b12d06f";
    const RECV: &str = "0x10a0847c2d170008ddca7c3a688124f493630032";
    const DATA: &str = "0xc11695480000000000000000000000004b274e4a9af31c20ed4151769a88ffe63d9439960000000000000000000000008510211a852f0c5994051dd85eaef73112a82eb5000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000bad4de000000000000000000000000607816a600000000000000000000000000000000000000000000000000000000000002200000000000000000000000000000000000000000000000000000001146aa2600000000000000000000000000000000000000000000000000000000000001bc9b000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000482579f93dc13e6b434e38b5a0447ca543d88a4600000000000000000000000000000000000000000000000000000000000000c42df546f40000000000000000000000004b274e4a9af31c20ed4151769a88ffe63d943996000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000007d93f93d41604572119e4be7757a7a4a43705f080000000000000000000000000000000000000000000000003782dace9d90000000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082b5a61569b5898ac347c82a594c86699f1981aa88ca46a6a00b8e4f27b3d17bdf3714e7c0ca6a8023b37cca556602fce7dc7daac3fcee1ab04bbb3b94c10dec301cc57266db6567aa073efaa1fa6669bdc6f0877b0aeab4e33d18cb08b8877f08931abf427f11bade042177db48ca956feb114d6f5d56d1f5889047189562ec545e1c000000000000000000000000000000000000000000000000000000000000";

    const ADDRESS_A: &str = "0x6856ccf24beb7ed77f1f24eee5742f7ece5557e2";
    const ADDRESS_B: &str = "0xb1dd690cc9af7bb1a906a9b5a94f94191cc553ce";
    const STORAGE_KEY: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";

    let input: TransactionEip2930 = TransactionEip2930 {
      chain_id: 0x1,
      from: SEND.parse().unwrap(),
      to: Some(RECV.parse().unwrap()),
      nonce: 0x18bf,
      gas_limit: "0x48a94".parse().unwrap(),
      gas_price: "0x105e34df00".parse().unwrap(),
      value: Default::default(),
      input: DATA.parse().unwrap(),
      access_list: AccessList::from([
        Access {
          address: ADDRESS_A.parse().unwrap(),
          storage_keys: vec![STORAGE_KEY.parse().unwrap()],
        },
        Access {
          address: ADDRESS_B.parse().unwrap(),
          storage_keys: Vec::new(),
        },
      ]),
    };

    let tokens: &[Token] = &[
      Token::Struct {
        name: "TransactionEip2930",
        len: 9,
      },
      Token::Str("chainId"),
      Token::Str("0x1"),
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("to"),
      Token::Some,
      Token::Str(RECV),
      Token::Str("nonce"),
      Token::Str("0x18bf"),
      Token::Str("gas"),
      Token::Str("0x48a94"),
      Token::Str("gasPrice"),
      Token::Str("0x105e34df00"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str(DATA),
      Token::Str("accessList"),
      Token::Seq { len: Some(2) },
      Token::Struct {
        name: "Access",
        len: 2,
      },
      Token::Str("address"),
      Token::Str(ADDRESS_A),
      Token::Str("storageKeys"),
      Token::Seq { len: Some(1) },
      Token::Str(STORAGE_KEY),
      Token::SeqEnd,
      Token::StructEnd,
      Token::Struct {
        name: "Access",
        len: 2,
      },
      Token::Str("address"),
      Token::Str(ADDRESS_B),
      Token::Str("storageKeys"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&input.readable(), tokens);
  }
}
