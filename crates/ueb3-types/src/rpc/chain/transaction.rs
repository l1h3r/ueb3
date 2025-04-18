use serde::__private::de::Content;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use serde::de::IntoDeserializer as _;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::de::value::MapDeserializer;
use serde::ser::SerializeStruct as _;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::iter::Flatten;
use std::marker::PhantomData;
use std::vec::IntoIter;
use ueb3_serde::flatten::SerializeFlatten as _;
use ueb3_serde::maybe::Maybe;

use crate::eip::eip2718::TransactionType;
use crate::eth::H256;
use crate::eth::U64;
use crate::eth::U256;
use crate::rpc::TransactionHeader;
use crate::rpc::TransactionObject;
use crate::rpc::TransactionSigner;

// -----------------------------------------------------------------------------
// Ethereum Transaction Object
// -----------------------------------------------------------------------------

/// A transaction object.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Transaction {
  /// The common properties relevant to all transaction types.
  pub header: TransactionHeader,
  /// The type-specific transaction properties.
  pub object: TransactionObject,
  /// The properties relevant to the creator of the transaction signature.
  pub signer: TransactionSigner,
}

impl Transaction {
  /// Returns `true` if `self` is a legacy transaction.
  #[inline]
  pub const fn is_legacy(&self) -> bool {
    self.object.is_legacy()
  }

  /// Returns `true` if `self` is an EIP-2930 transaction.
  #[inline]
  pub const fn is_eip2930(&self) -> bool {
    self.object.is_eip2930()
  }

  /// Returns `true` if `self` is an EIP-1559 transaction.
  #[inline]
  pub const fn is_eip1559(&self) -> bool {
    self.object.is_eip1559()
  }

  /// Returns `true` if `self` is an EIP-4844 transaction.
  #[inline]
  pub const fn is_eip4844(&self) -> bool {
    self.object.is_eip4844()
  }

  /// Returns `true` if `self` is an EIP-7702 transaction.
  #[inline]
  pub const fn is_eip7702(&self) -> bool {
    self.object.is_eip7702()
  }

  /// Returns the [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  #[inline]
  pub const fn transaction_type(&self) -> TransactionType {
    self.object.transaction_type()
  }
}

impl<'de> Deserialize<'de> for Transaction {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl<'de> Visitor<'de> for __Visitor {
      type Value = Transaction;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a transaction object")
      }

      fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        #[derive(Deserialize)]
        #[serde(field_identifier)]
        enum Field<'a> {
          #[serde(rename = "hash")]
          Hash,
          #[serde(rename = "transactionIndex")]
          Index,
          #[serde(rename = "blockHash")]
          BlockHash,
          #[serde(rename = "blockNumber")]
          BlockNumber,
          #[serde(rename = "v")]
          SigV,
          #[serde(rename = "r")]
          SigR,
          #[serde(rename = "s")]
          SigS,
          Other(&'a str),
        }

        let mut hash: Maybe<A, H256> = Maybe::NONE;
        let mut index: Maybe<A, Option<U64>> = Maybe::NONE;
        let mut block_hash: Maybe<A, Option<H256>> = Maybe::NONE;
        let mut block_number: Maybe<A, Option<U64>> = Maybe::NONE;

        let mut v: Maybe<A, U64> = Maybe::NONE;
        let mut r: Maybe<A, U256> = Maybe::NONE;
        let mut s: Maybe<A, U256> = Maybe::NONE;

        let mut other: ContentVec<'de, A> = ContentVec::new();

        while let Some(field) = access.next_key()? {
          match field {
            // -----------------------------------------------------------------
            // Header Fields
            // -----------------------------------------------------------------
            Field::Hash => hash.set(&mut access, "hash")?,
            Field::Index => index.set(&mut access, "transactionIndex")?,
            Field::BlockHash => block_hash.set(&mut access, "blockHash")?,
            Field::BlockNumber => block_number.set(&mut access, "blockNumber")?,
            // -----------------------------------------------------------------
            // Signer Fields
            // -----------------------------------------------------------------
            Field::SigV => v.set(&mut access, "v")?,
            Field::SigR => r.set(&mut access, "r")?,
            Field::SigS => s.set(&mut access, "s")?,
            // -----------------------------------------------------------------
            // Object Fields
            // -----------------------------------------------------------------
            Field::Other(field) => other.push(field, access.next_value()?),
          }
        }

        let object: TransactionObject = other.deserialize()?;

        let header: TransactionHeader = TransactionHeader {
          hash: hash.get_required("hash")?,
          index: index.get_required("transactionIndex")?.map(U64::into_u64),
          block_hash: block_hash.get_required("blockHash")?,
          block_number: block_number.get_required("blockNumber")?.map(U64::into_u64),
        };

        let signer: TransactionSigner = TransactionSigner {
          v: v.get_required("v")?.into_u64(),
          r: r.get_required("r")?,
          s: s.get_required("s")?,
        };

        Ok(Transaction {
          header,
          object,
          signer,
        })
      }
    }

    deserializer.deserialize_map(__Visitor)
  }
}

impl Serialize for Transaction {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let fields: usize = 4 + self.object.fields() + 3;
    let mut state: S::SerializeStruct = serializer.serialize_struct("Transaction", fields)?;

    state.serialize_field("hash", &self.header.hash)?;
    state.serialize_field("transactionIndex", &self.header.index.map(U64::from_u64))?;
    state.serialize_field("blockHash", &self.header.block_hash)?;
    state.serialize_field("blockNumber", &self.header.block_number.map(U64::from_u64))?;

    self.object.serialize_fields(&mut state)?;

    state.serialize_field("v", &U64::from_u64(self.signer.v))?;
    state.serialize_field("r", &self.signer.r)?;
    state.serialize_field("s", &self.signer.s)?;

    state.end()
  }
}

// -----------------------------------------------------------------------------
// Serde Utilities
// -----------------------------------------------------------------------------

type ContentItem<'de> = Option<(&'de str, Content<'de>)>;
type ContentIter<'de> = Flatten<IntoIter<ContentItem<'de>>>;

struct ContentVec<'de, A> {
  content: Vec<ContentItem<'de>>,
  phantom: PhantomData<A>,
}

impl<'de, A> ContentVec<'de, A> {
  const FIELDS: usize = 15;

  #[inline]
  fn new() -> Self {
    Self {
      content: Vec::with_capacity(Self::FIELDS),
      phantom: PhantomData,
    }
  }

  #[inline]
  fn push(&mut self, field: &'de str, value: Content<'de>) {
    self.content.push(Some((field, value)));
  }

  #[inline]
  fn into_iter(self) -> ContentIter<'de> {
    self.content.into_iter().flatten()
  }

  #[inline]
  fn take_type(&mut self) -> Result<TransactionType, A::Error>
  where
    A: MapAccess<'de>,
  {
    for entry in &mut self.content {
      if matches!(entry, Some(("type", _))) {
        let Some(("type", value)) = entry.take() else {
          unreachable!()
        };

        return Deserialize::deserialize(value.into_deserializer());
      }
    }

    Err(A::Error::missing_field("type"))
  }

  #[inline]
  fn deserialize(mut self) -> Result<TransactionObject, A::Error>
  where
    A: MapAccess<'de>,
  {
    let tx_type: TransactionType = self.take_type()?;
    let mut iter: ContentIter<'de> = self.into_iter();
    let deserializer: MapDeserializer<'_, &mut _, A::Error> = MapDeserializer::new(iter.by_ref());

    let object: TransactionObject = match tx_type {
      TransactionType::Legacy => {
        Deserialize::deserialize(deserializer).map(TransactionObject::Legacy)?
      }
      TransactionType::Eip2930 => {
        Deserialize::deserialize(deserializer).map(TransactionObject::Eip2930)?
      }
      TransactionType::Eip1559 => {
        Deserialize::deserialize(deserializer).map(TransactionObject::Eip1559)?
      }
      TransactionType::Eip4844 => {
        Deserialize::deserialize(deserializer).map(TransactionObject::Eip4844)?
      }
      TransactionType::Eip7702 => {
        Deserialize::deserialize(deserializer).map(TransactionObject::Eip7702)?
      }
    };

    if let Some((remain, _)) = iter.next() {
      return Err(A::Error::unknown_field(remain, &[]));
    }

    Ok(object)
  }
}

#[cfg(test)]
mod tests {
  use serde_json::from_str;
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip2718::TransactionType;
  use crate::rpc::Transaction;

  const LEGACY_JSON: &str = r#"
  {
    "blockHash": "0x4e3a3754410177e6937ef1f84bba68ea139e8d1a2258c5f85db9f1cd715a1bdd",
    "blockNumber": "0xb443",
    "from": "0xa1e4380a3b1f749673e270229993ee55f35663b4",
    "gas": "0x5208",
    "gasPrice": "0x2d79883d2000",
    "hash": "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060",
    "input": "0x",
    "nonce": "0x0",
    "to": "0x5df9b87991262f6ba471f09758cde1c0fc1de734",
    "transactionIndex": "0x0",
    "value": "0x7a69",
    "type": "0x0",
    "v": "0x1c",
    "r": "0x88ff6cf0fefd94db46111149ae4bfc179e9b94721fffd821d38d16464b3f71d0",
    "s": "0x45e0aff800961cfce805daef7016b9b675c137a6a41a548f7b60a3484c06a33a"
  }
  "#;

  const LEGACY_TOKENS: &'static [Token] = &[
    Token::Struct {
      name: "Transaction",
      len: 15,
    },
    Token::Str("hash"),
    Token::Str("0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060"),
    Token::Str("transactionIndex"),
    Token::Some,
    Token::Str("0x0"),
    Token::Str("blockHash"),
    Token::Some,
    Token::Str("0x4e3a3754410177e6937ef1f84bba68ea139e8d1a2258c5f85db9f1cd715a1bdd"),
    Token::Str("blockNumber"),
    Token::Some,
    Token::Str("0xb443"),
    Token::BorrowedStr("type"),
    Token::UnitVariant {
      name: "TransactionType",
      variant: "0x0",
    },
    Token::BorrowedStr("from"),
    Token::Str("0xa1e4380a3b1f749673e270229993ee55f35663b4"),
    Token::BorrowedStr("to"),
    Token::Some,
    Token::Str("0x5df9b87991262f6ba471f09758cde1c0fc1de734"),
    Token::BorrowedStr("nonce"),
    Token::Str("0x0"),
    Token::BorrowedStr("gas"),
    Token::Str("0x5208"),
    Token::BorrowedStr("gasPrice"),
    Token::Str("0x2d79883d2000"),
    Token::BorrowedStr("value"),
    Token::Str("0x7a69"),
    Token::BorrowedStr("input"),
    Token::Str("0x"),
    Token::Str("v"),
    Token::Str("0x1c"),
    Token::Str("r"),
    Token::Str("0x88ff6cf0fefd94db46111149ae4bfc179e9b94721fffd821d38d16464b3f71d0"),
    Token::Str("s"),
    Token::Str("0x45e0aff800961cfce805daef7016b9b675c137a6a41a548f7b60a3484c06a33a"),
    Token::StructEnd,
  ];

  const EIP_2930_JSON: &str = r#"
  {
    "blockHash": "0xc1f52bc3eeca1f02af8f423c08945c8038db89286fcab3aff70f88f176fe49de",
    "blockNumber": "0xbad4b1",
    "from": "0x1ced2cef30d40bb3617f8d455071b69f3b12d06f",
    "gas": "0x48a94",
    "gasPrice": "0x105e34df00",
    "hash": "0x851bad0415758075a1eb86776749c829b866d43179c57c3e4a4b9359a0358231",
    "input": "0xc11695480000000000000000000000004b274e4a9af31c20ed4151769a88ffe63d9439960000000000000000000000008510211a852f0c5994051dd85eaef73112a82eb5000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000bad4de000000000000000000000000607816a600000000000000000000000000000000000000000000000000000000000002200000000000000000000000000000000000000000000000000000001146aa2600000000000000000000000000000000000000000000000000000000000001bc9b000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000482579f93dc13e6b434e38b5a0447ca543d88a4600000000000000000000000000000000000000000000000000000000000000c42df546f40000000000000000000000004b274e4a9af31c20ed4151769a88ffe63d943996000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000007d93f93d41604572119e4be7757a7a4a43705f080000000000000000000000000000000000000000000000003782dace9d90000000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082b5a61569b5898ac347c82a594c86699f1981aa88ca46a6a00b8e4f27b3d17bdf3714e7c0ca6a8023b37cca556602fce7dc7daac3fcee1ab04bbb3b94c10dec301cc57266db6567aa073efaa1fa6669bdc6f0877b0aeab4e33d18cb08b8877f08931abf427f11bade042177db48ca956feb114d6f5d56d1f5889047189562ec545e1c000000000000000000000000000000000000000000000000000000000000",
    "nonce": "0x18bf",
    "to": "0x10a0847c2d170008ddca7c3a688124f493630032",
    "transactionIndex": "0xe",
    "value": "0x0",
    "type": "0x1",
    "accessList": [
      {
        "address": "0x6856ccf24beb7ed77f1f24eee5742f7ece5557e2",
        "storageKeys": [
          "0x0000000000000000000000000000000000000000000000000000000000000001"
        ]
      },
      {
        "address": "0xb1dd690cc9af7bb1a906a9b5a94f94191cc553ce",
        "storageKeys": []
      }
    ],
    "chainId": "0x1",
    "v": "0x0",
    "r": "0xd52f3dbcad3530e73fcef6f4a75329b569a8903bf6d8109a960901f251a37af3",
    "s": "0xecf570e0c0ffa6efdc6e6e49be764b6a1a77e47de7bb99e167544ffbbcd65bc",
    "yParity": "0x0"
  }
  "#;

  const EIP_2930_TOKENS: &'static [Token] = &[
    Token::Struct {
      name: "Transaction",
      len: 18,
    },
    Token::BorrowedStr("hash"),
    Token::Str("0x851bad0415758075a1eb86776749c829b866d43179c57c3e4a4b9359a0358231"),
    Token::BorrowedStr("transactionIndex"),
    Token::Some,
    Token::Str("0xe"),
    Token::BorrowedStr("blockHash"),
    Token::Some,
    Token::Str("0xc1f52bc3eeca1f02af8f423c08945c8038db89286fcab3aff70f88f176fe49de"),
    Token::BorrowedStr("blockNumber"),
    Token::Some,
    Token::Str("0xbad4b1"),
    Token::BorrowedStr("type"),
    Token::UnitVariant {
      name: "TransactionType",
      variant: "0x1",
    },
    Token::BorrowedStr("chainId"),
    Token::Str("0x1"),
    Token::BorrowedStr("from"),
    Token::Str("0x1ced2cef30d40bb3617f8d455071b69f3b12d06f"),
    Token::BorrowedStr("to"),
    Token::Some,
    Token::Str("0x10a0847c2d170008ddca7c3a688124f493630032"),
    Token::BorrowedStr("nonce"),
    Token::Str("0x18bf"),
    Token::BorrowedStr("gas"),
    Token::Str("0x48a94"),
    Token::BorrowedStr("gasPrice"),
    Token::Str("0x105e34df00"),
    Token::BorrowedStr("value"),
    Token::Str("0x0"),
    Token::BorrowedStr("input"),
    Token::Str(
      "0xc11695480000000000000000000000004b274e4a9af31c20ed4151769a88ffe63d9439960000000000000000000000008510211a852f0c5994051dd85eaef73112a82eb5000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000bad4de000000000000000000000000607816a600000000000000000000000000000000000000000000000000000000000002200000000000000000000000000000000000000000000000000000001146aa2600000000000000000000000000000000000000000000000000000000000001bc9b000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000482579f93dc13e6b434e38b5a0447ca543d88a4600000000000000000000000000000000000000000000000000000000000000c42df546f40000000000000000000000004b274e4a9af31c20ed4151769a88ffe63d943996000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000007d93f93d41604572119e4be7757a7a4a43705f080000000000000000000000000000000000000000000000003782dace9d90000000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082b5a61569b5898ac347c82a594c86699f1981aa88ca46a6a00b8e4f27b3d17bdf3714e7c0ca6a8023b37cca556602fce7dc7daac3fcee1ab04bbb3b94c10dec301cc57266db6567aa073efaa1fa6669bdc6f0877b0aeab4e33d18cb08b8877f08931abf427f11bade042177db48ca956feb114d6f5d56d1f5889047189562ec545e1c000000000000000000000000000000000000000000000000000000000000",
    ),
    Token::BorrowedStr("yParity"),
    Token::UnitVariant {
      name: "YParity",
      variant: "0x0",
    },
    Token::BorrowedStr("accessList"),
    Token::Seq { len: Some(2) },
    Token::Struct {
      name: "Access",
      len: 2,
    },
    Token::BorrowedStr("address"),
    Token::Str("0x6856ccf24beb7ed77f1f24eee5742f7ece5557e2"),
    Token::BorrowedStr("storageKeys"),
    Token::Seq { len: Some(1) },
    Token::Str("0x0000000000000000000000000000000000000000000000000000000000000001"),
    Token::SeqEnd,
    Token::StructEnd,
    Token::Struct {
      name: "Access",
      len: 2,
    },
    Token::BorrowedStr("address"),
    Token::Str("0xb1dd690cc9af7bb1a906a9b5a94f94191cc553ce"),
    Token::BorrowedStr("storageKeys"),
    Token::Seq { len: Some(0) },
    Token::SeqEnd,
    Token::StructEnd,
    Token::SeqEnd,
    Token::BorrowedStr("v"),
    Token::Str("0x0"),
    Token::BorrowedStr("r"),
    Token::Str("0xd52f3dbcad3530e73fcef6f4a75329b569a8903bf6d8109a960901f251a37af3"),
    Token::BorrowedStr("s"),
    Token::Str("0xecf570e0c0ffa6efdc6e6e49be764b6a1a77e47de7bb99e167544ffbbcd65bc"),
    Token::StructEnd,
  ];

  const EIP_1559_JSON: &str = r#"
  {
    "blockHash": "0xa32d159805750cbe428b799a49b85dcb2300f61d806786f317260e721727d162",
    "blockNumber": "0xc5d489",
    "from": "0x2e419a06feb47d5f640636a55a814757fa10edf9",
    "gas": "0xb886",
    "gasPrice": "0x755d7a88e",
    "maxFeePerGas": "0x789ff9700",
    "maxPriorityFeePerGas": "0x712ca0300",
    "hash": "0xed382cb554ad10e94921d263a56c670669d6c380bbdacdbf96fed625b7132a1d",
    "input": "0x095ea7b30000000000000000000000007645eec8bb51862a5aa855c40971b2877dae81afffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    "nonce": "0x105",
    "to": "0x491d6b7d6822d5d4bc88a1264e1b47791fd8e904",
    "transactionIndex": "0x62",
    "value": "0x0",
    "type": "0x2",
    "accessList": [],
    "chainId": "0x1",
    "v": "0x1",
    "r": "0x416470241b7db89c67526881b6fd8e145416b294a35bf4280d3079f6308c2d11",
    "s": "0x2c0af1cc55c22c0bab79ec083801da63253453156356fcd4291f50d0f425a0ee",
    "yParity": "0x1"
  }
  "#;

  const EIP_1559_TOKENS: &'static [Token] = &[
    Token::Struct {
      name: "Transaction",
      len: 20,
    },
    Token::BorrowedStr("hash"),
    Token::Str("0xed382cb554ad10e94921d263a56c670669d6c380bbdacdbf96fed625b7132a1d"),
    Token::BorrowedStr("transactionIndex"),
    Token::Some,
    Token::Str("0x62"),
    Token::BorrowedStr("blockHash"),
    Token::Some,
    Token::Str("0xa32d159805750cbe428b799a49b85dcb2300f61d806786f317260e721727d162"),
    Token::BorrowedStr("blockNumber"),
    Token::Some,
    Token::Str("0xc5d489"),
    Token::BorrowedStr("type"),
    Token::UnitVariant {
      name: "TransactionType",
      variant: "0x2",
    },
    Token::BorrowedStr("chainId"),
    Token::Str("0x1"),
    Token::BorrowedStr("from"),
    Token::Str("0x2e419a06feb47d5f640636a55a814757fa10edf9"),
    Token::BorrowedStr("to"),
    Token::Some,
    Token::Str("0x491d6b7d6822d5d4bc88a1264e1b47791fd8e904"),
    Token::BorrowedStr("nonce"),
    Token::Str("0x105"),
    Token::BorrowedStr("gas"),
    Token::Str("0xb886"),
    Token::BorrowedStr("gasPrice"),
    Token::Str("0x755d7a88e"),
    Token::BorrowedStr("maxPriorityFeePerGas"),
    Token::Str("0x712ca0300"),
    Token::BorrowedStr("maxFeePerGas"),
    Token::Str("0x789ff9700"),
    Token::BorrowedStr("value"),
    Token::Str("0x0"),
    Token::BorrowedStr("input"),
    Token::Str(
      "0x095ea7b30000000000000000000000007645eec8bb51862a5aa855c40971b2877dae81afffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ),
    Token::BorrowedStr("yParity"),
    Token::UnitVariant {
      name: "YParity",
      variant: "0x1",
    },
    Token::BorrowedStr("accessList"),
    Token::Seq { len: Some(0) },
    Token::SeqEnd,
    Token::BorrowedStr("v"),
    Token::Str("0x1"),
    Token::BorrowedStr("r"),
    Token::Str("0x416470241b7db89c67526881b6fd8e145416b294a35bf4280d3079f6308c2d11"),
    Token::BorrowedStr("s"),
    Token::Str("0x2c0af1cc55c22c0bab79ec083801da63253453156356fcd4291f50d0f425a0ee"),
    Token::StructEnd,
  ];

  const EIP_4844_JSON: &str = r#"
  {
    "blockHash": "0xa2917e0758c98640d868182838c93bb12f0d07b6b17efe6b62d9df42c7643791",
    "blockNumber": "0x1286d1d",
    "from": "0x40c35d4faf69234986cb599890c2d2ef546074a9",
    "gas": "0x5208",
    "gasPrice": "0xd364c1438",
    "maxFeePerGas": "0x1d1e4e4ea00",
    "maxPriorityFeePerGas": "0x3b9aca00",
    "maxFeePerBlobGas": "0x3b9aca00",
    "hash": "0x9c1fbda4f649ac806ab0faefbe94e1a60282eb374ead6aa01bac042f52b28a8c",
    "input": "0x",
    "nonce": "0x0",
    "to": "0x0000000000000000000000000000000000000000",
    "transactionIndex": "0x46",
    "value": "0x0",
    "type": "0x3",
    "accessList": [],
    "chainId": "0x1",
    "blobVersionedHashes": [
      "0x0175f564b393e44640ecffddce4010a42c1c966413987d8a59a253ef64a0c5cf"
    ],
    "v": "0x1",
    "r": "0x4169d4bf9df18182d86715d4ec031c20aded1a4a2613f57e0c0169e16cf763c2",
    "s": "0x6fc81d18602b8a1e065193488a7c943dfd780adb7b72dfc332de7d68605986e",
    "yParity": "0x1"
  }
  "#;

  const EIP_4844_TOKENS: &'static [Token] = &[
    Token::Struct {
      name: "Transaction",
      len: 22,
    },
    Token::BorrowedStr("hash"),
    Token::Str("0x9c1fbda4f649ac806ab0faefbe94e1a60282eb374ead6aa01bac042f52b28a8c"),
    Token::BorrowedStr("transactionIndex"),
    Token::Some,
    Token::Str("0x46"),
    Token::BorrowedStr("blockHash"),
    Token::Some,
    Token::Str("0xa2917e0758c98640d868182838c93bb12f0d07b6b17efe6b62d9df42c7643791"),
    Token::BorrowedStr("blockNumber"),
    Token::Some,
    Token::Str("0x1286d1d"),
    Token::BorrowedStr("type"),
    Token::UnitVariant {
      name: "TransactionType",
      variant: "0x3",
    },
    Token::BorrowedStr("chainId"),
    Token::Str("0x1"),
    Token::BorrowedStr("from"),
    Token::Str("0x40c35d4faf69234986cb599890c2d2ef546074a9"),
    Token::BorrowedStr("to"),
    Token::Some,
    Token::Str("0x0000000000000000000000000000000000000000"),
    Token::BorrowedStr("nonce"),
    Token::Str("0x0"),
    Token::BorrowedStr("gas"),
    Token::Str("0x5208"),
    Token::BorrowedStr("gasPrice"),
    Token::Str("0xd364c1438"),
    Token::BorrowedStr("maxPriorityFeePerGas"),
    Token::Str("0x3b9aca00"),
    Token::BorrowedStr("maxFeePerGas"),
    Token::Str("0x1d1e4e4ea00"),
    Token::BorrowedStr("maxFeePerBlobGas"),
    Token::Str("0x3b9aca00"),
    Token::BorrowedStr("value"),
    Token::Str("0x0"),
    Token::BorrowedStr("input"),
    Token::Str("0x"),
    Token::BorrowedStr("yParity"),
    Token::UnitVariant {
      name: "YParity",
      variant: "0x1",
    },
    Token::BorrowedStr("accessList"),
    Token::Seq { len: Some(0) },
    Token::SeqEnd,
    Token::BorrowedStr("blobVersionedHashes"),
    Token::Seq { len: Some(1) },
    Token::Str("0x0175f564b393e44640ecffddce4010a42c1c966413987d8a59a253ef64a0c5cf"),
    Token::SeqEnd,
    Token::BorrowedStr("v"),
    Token::Str("0x1"),
    Token::BorrowedStr("r"),
    Token::Str("0x4169d4bf9df18182d86715d4ec031c20aded1a4a2613f57e0c0169e16cf763c2"),
    Token::BorrowedStr("s"),
    Token::Str("0x6fc81d18602b8a1e065193488a7c943dfd780adb7b72dfc332de7d68605986e"),
    Token::StructEnd,
  ];

  const EIP_7702_JSON: &str = r#"
  {
    "blockHash": "0xee4aa6792fdf3fdf2fbdf570b69e56b603c078162e624593485da6961d441f59",
    "blockNumber": "0x1564582",
    "from": "0x86628a07320098baa038848e88a4217037fb898b",
    "gas": "0x11017",
    "gasPrice": "0x2c55791e0",
    "maxFeePerGas": "0x2f2f04d66",
    "maxPriorityFeePerGas": "0x16673a77d",
    "hash": "0xafc2e5a0c77aacd5ac75717fa95005e506534909f4a3db008bfe7a8e32d76569",
    "input": "0x",
    "nonce": "0x2",
    "to": "0x86628a07320098baa038848e88a4217037fb898b",
    "transactionIndex": "0x74",
    "value": "0x0",
    "type": "0x4",
    "accessList": [],
    "chainId": "0x1",
    "authorizationList": [
      {
        "chainId": "0x1",
        "address": "0x0000000000000000000000000000000000000000",
        "nonce": "0x3",
        "yParity": "0x1",
        "r": "0x28da8576c6e73b424e8cf57e46c683fae141ecbb7150e8ede3b673e82a6f90a6",
        "s": "0x1ca5eb17d037909806a3c5cca595633369cdcb2f59b3916171e85782c01a296d"
      }
    ],
    "v": "0x0",
    "r": "0x2b6bb613f4d1b39b148c93dc981c8458eb4fd98b2b681d3fc63b3b7f1c85e99",
    "s": "0x2dba0d93c9c072cb1fe8b0ffafd7e666f6894c8b4b20989840c01bd3db4bc7ed",
    "yParity": "0x0"
  }
  "#;

  const EIP_7702_TOKENS: &'static [Token] = &[
    Token::Struct {
      name: "Transaction",
      len: 21,
    },
    Token::BorrowedStr("hash"),
    Token::Str("0xafc2e5a0c77aacd5ac75717fa95005e506534909f4a3db008bfe7a8e32d76569"),
    Token::BorrowedStr("transactionIndex"),
    Token::Some,
    Token::Str("0x74"),
    Token::BorrowedStr("blockHash"),
    Token::Some,
    Token::Str("0xee4aa6792fdf3fdf2fbdf570b69e56b603c078162e624593485da6961d441f59"),
    Token::BorrowedStr("blockNumber"),
    Token::Some,
    Token::Str("0x1564582"),
    Token::BorrowedStr("type"),
    Token::UnitVariant {
      name: "TransactionType",
      variant: "0x4",
    },
    Token::BorrowedStr("chainId"),
    Token::Str("0x1"),
    Token::BorrowedStr("from"),
    Token::Str("0x86628a07320098baa038848e88a4217037fb898b"),
    Token::BorrowedStr("to"),
    Token::Some,
    Token::Str("0x86628a07320098baa038848e88a4217037fb898b"),
    Token::BorrowedStr("nonce"),
    Token::Str("0x2"),
    Token::BorrowedStr("gas"),
    Token::Str("0x11017"),
    Token::BorrowedStr("gasPrice"),
    Token::Str("0x2c55791e0"),
    Token::BorrowedStr("maxPriorityFeePerGas"),
    Token::Str("0x16673a77d"),
    Token::BorrowedStr("maxFeePerGas"),
    Token::Str("0x2f2f04d66"),
    Token::BorrowedStr("value"),
    Token::Str("0x0"),
    Token::BorrowedStr("input"),
    Token::Str("0x"),
    Token::BorrowedStr("yParity"),
    Token::UnitVariant {
      name: "YParity",
      variant: "0x0",
    },
    Token::BorrowedStr("accessList"),
    Token::Seq { len: Some(0) },
    Token::SeqEnd,
    Token::BorrowedStr("authorizationList"),
    Token::Seq { len: Some(1) },
    Token::Struct {
      name: "Authorization",
      len: 6,
    },
    Token::BorrowedStr("chainId"),
    Token::Str("0x1"),
    Token::BorrowedStr("address"),
    Token::Str("0x0000000000000000000000000000000000000000"),
    Token::BorrowedStr("nonce"),
    Token::Str("0x3"),
    Token::BorrowedStr("yParity"),
    Token::UnitVariant {
      name: "YParity",
      variant: "0x1",
    },
    Token::BorrowedStr("r"),
    Token::Str("0x28da8576c6e73b424e8cf57e46c683fae141ecbb7150e8ede3b673e82a6f90a6"),
    Token::BorrowedStr("s"),
    Token::Str("0x1ca5eb17d037909806a3c5cca595633369cdcb2f59b3916171e85782c01a296d"),
    Token::StructEnd,
    Token::SeqEnd,
    Token::BorrowedStr("v"),
    Token::Str("0x0"),
    Token::BorrowedStr("r"),
    Token::Str("0x2b6bb613f4d1b39b148c93dc981c8458eb4fd98b2b681d3fc63b3b7f1c85e99"),
    Token::BorrowedStr("s"),
    Token::Str("0x2dba0d93c9c072cb1fe8b0ffafd7e666f6894c8b4b20989840c01bd3db4bc7ed"),
    Token::StructEnd,
  ];

  #[test]
  fn test_serde_legacy() {
    let transaction: Transaction = from_str(LEGACY_JSON).unwrap();

    assert_eq!(transaction.is_legacy(), true);
    assert_eq!(transaction.transaction_type(), TransactionType::Legacy);

    assert_tokens(&transaction.readable(), LEGACY_TOKENS);
  }

  #[test]
  fn test_serde_eip2930() {
    let transaction: Transaction = from_str(EIP_2930_JSON).unwrap();

    assert_eq!(transaction.is_eip2930(), true);
    assert_eq!(transaction.transaction_type(), TransactionType::Eip2930);

    assert_tokens(&transaction.readable(), EIP_2930_TOKENS);
  }

  #[test]
  fn test_serde_eip1559() {
    let transaction: Transaction = from_str(EIP_1559_JSON).unwrap();

    assert_eq!(transaction.is_eip1559(), true);
    assert_eq!(transaction.transaction_type(), TransactionType::Eip1559);

    assert_tokens(&transaction.readable(), EIP_1559_TOKENS);
  }

  #[test]
  fn test_serde_eip4844() {
    let transaction: Transaction = from_str(EIP_4844_JSON).unwrap();

    assert_eq!(transaction.is_eip4844(), true);
    assert_eq!(transaction.transaction_type(), TransactionType::Eip4844);

    assert_tokens(&transaction.readable(), EIP_4844_TOKENS);
  }

  #[test]
  fn test_serde_eip7702() {
    let transaction: Transaction = from_str(EIP_7702_JSON).unwrap();

    assert_eq!(transaction.is_eip7702(), true);
    assert_eq!(transaction.transaction_type(), TransactionType::Eip7702);

    assert_tokens(&transaction.readable(), EIP_7702_TOKENS);
  }
}
