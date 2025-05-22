use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten;

use crate::eip::eip2718::TransactionType;
use crate::eip::eip2930::AccessList;
use crate::eip::eip7702::AuthorizationList;
use crate::eth::Address;
use crate::eth::H256;
use crate::eth::Slice;
use crate::eth::U256;
use crate::eth::Wei;
use crate::rpc::TransactionEip1559;
use crate::rpc::TransactionEip2930;
use crate::rpc::TransactionEip4844;
use crate::rpc::TransactionEip7702;
use crate::rpc::TransactionLegacy;

macro_rules! select {
  ($this:expr, $inner:ident => $return:expr) => {
    match $this {
      Self::Legacy($inner) => $return,
      Self::Eip2930($inner) => $return,
      Self::Eip1559($inner) => $return,
      Self::Eip4844($inner) => $return,
      Self::Eip7702($inner) => $return,
    }
  };
}

// -----------------------------------------------------------------------------
// Ethereum Transaction Payload
// -----------------------------------------------------------------------------

/// A set of type-specific transaction properties.
#[derive(Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, tag = "type")]
pub enum TransactionPayload {
  /// Legacy transaction object.
  #[serde(rename = "0x0", alias = "0x00")]
  Legacy(TransactionLegacy),
  /// EIP-2930 transaction object.
  #[serde(rename = "0x1", alias = "0x01")]
  Eip2930(TransactionEip2930),
  /// EIP-1559 transaction object.
  #[serde(rename = "0x2", alias = "0x02")]
  Eip1559(TransactionEip1559),
  /// EIP-4844 transaction object.
  #[serde(rename = "0x3", alias = "0x03")]
  Eip4844(TransactionEip4844),
  /// EIP-7702 transaction object.
  #[serde(rename = "0x4", alias = "0x04")]
  Eip7702(TransactionEip7702),
}

impl TransactionPayload {
  /// Returns `true` if `self` is a legacy transaction.
  #[inline]
  pub const fn is_legacy(&self) -> bool {
    matches!(self, Self::Legacy(_))
  }

  /// Returns `true` if `self` is an EIP-2930 transaction.
  #[inline]
  pub const fn is_eip2930(&self) -> bool {
    matches!(self, Self::Eip2930(_))
  }

  /// Returns `true` if `self` is an EIP-1559 transaction.
  #[inline]
  pub const fn is_eip1559(&self) -> bool {
    matches!(self, Self::Eip1559(_))
  }

  /// Returns `true` if `self` is an EIP-4844 transaction.
  #[inline]
  pub const fn is_eip4844(&self) -> bool {
    matches!(self, Self::Eip4844(_))
  }

  /// Returns `true` if `self` is an EIP-7702 transaction.
  #[inline]
  pub const fn is_eip7702(&self) -> bool {
    matches!(self, Self::Eip7702(_))
  }

  /// Returns the [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  #[inline]
  pub const fn transaction_type(&self) -> TransactionType {
    match self {
      Self::Legacy(_) => TransactionLegacy::TYPE,
      Self::Eip2930(_) => TransactionEip2930::TYPE,
      Self::Eip1559(_) => TransactionEip1559::TYPE,
      Self::Eip4844(_) => TransactionEip4844::TYPE,
      Self::Eip7702(_) => TransactionEip7702::TYPE,
    }
  }

  /// Returns a unique identifier for which chain the transaction occured on.
  #[inline]
  pub const fn chain_id(&self) -> Option<u64> {
    match self {
      Self::Legacy(inner) => inner.chain_id,
      Self::Eip2930(inner) => Some(inner.chain_id),
      Self::Eip1559(inner) => Some(inner.chain_id),
      Self::Eip4844(inner) => Some(inner.chain_id),
      Self::Eip7702(inner) => Some(inner.chain_id),
    }
  }

  /// Returns the address of the transaction sender.
  #[inline]
  pub const fn sender(&self) -> Address {
    select!(self, inner => inner.from)
  }

  /// Returns the address of the transaction receiver.
  #[inline]
  pub const fn receiver(&self) -> Option<Address> {
    select!(self, inner => inner.to)
  }

  /// Returns the number of prior transactions made by the sender.
  #[inline]
  pub const fn nonce(&self) -> u64 {
    select!(self, inner => inner.nonce)
  }

  /// Returns the total amount of gas provided by the sender.
  #[inline]
  pub const fn gas_limit(&self) -> U256 {
    select!(self, inner => inner.gas_limit)
  }

  /// Returns the effective gas price paid by the sender.
  #[inline]
  pub const fn gas_price(&self) -> Option<Wei> {
    match self {
      Self::Legacy(inner) => Some(inner.gas_price),
      Self::Eip2930(inner) => Some(inner.gas_price),
      Self::Eip1559(inner) => inner.gas_price,
      Self::Eip4844(inner) => inner.gas_price,
      Self::Eip7702(inner) => inner.gas_price,
    }
  }

  /// Returns the maximum fee per gas the sender is willing to pay to miners.
  #[inline]
  pub const fn max_priority_fee_per_gas(&self) -> Option<Wei> {
    match self {
      Self::Legacy(_) => None,
      Self::Eip2930(_) => None,
      Self::Eip1559(inner) => Some(inner.max_priority_fee_per_gas),
      Self::Eip4844(inner) => Some(inner.max_priority_fee_per_gas),
      Self::Eip7702(inner) => Some(inner.max_priority_fee_per_gas),
    }
  }

  /// Returns the maximum total fee per gas the sender is willing to pay
  /// (includes the network / base fee and miner / priority fee).
  #[inline]
  pub const fn max_fee_per_gas(&self) -> Option<Wei> {
    match self {
      Self::Legacy(_) => None,
      Self::Eip2930(_) => None,
      Self::Eip1559(inner) => Some(inner.max_fee_per_gas),
      Self::Eip4844(inner) => Some(inner.max_fee_per_gas),
      Self::Eip7702(inner) => Some(inner.max_fee_per_gas),
    }
  }

  /// Returns the maximum total fee per gas the sender is willing to pay for blob gas.
  #[inline]
  pub const fn max_fee_per_blob_gas(&self) -> Option<Wei> {
    let Self::Eip4844(inner) = self else {
      return None;
    };

    Some(inner.max_fee_per_blob_gas)
  }

  /// Returns the amount of ETH transferred in the transaction.
  #[inline]
  pub const fn value(&self) -> Wei {
    select!(self, inner => inner.value)
  }

  /// Returns the additional data included with the transaction.
  #[inline]
  pub const fn input(&self) -> &Slice {
    select!(self, inner => inner.input.as_slice())
  }

  /// Returns a list of addresses/storage keys that the transaction plans to access.
  #[inline]
  pub const fn access_list(&self) -> Option<&AccessList> {
    match self {
      Self::Legacy(_) => None,
      Self::Eip2930(inner) => Some(&inner.access_list),
      Self::Eip1559(inner) => Some(&inner.access_list),
      Self::Eip4844(inner) => Some(&inner.access_list),
      Self::Eip7702(inner) => Some(&inner.access_list),
    }
  }

  /// Returns a list of versioned blob hashes associated with the transaction's data blobs.
  #[inline]
  pub const fn blob_versioned_hashes(&self) -> Option<&[H256]> {
    let Self::Eip4844(inner) = self else {
      return None;
    };

    Some(inner.blob_versioned_hashes.as_slice())
  }

  /// Returns a list of authorizations for the transaction.
  #[inline]
  pub const fn authorization_list(&self) -> Option<&AuthorizationList> {
    let Self::Eip7702(inner) = self else {
      return None;
    };

    Some(&inner.authorization_list)
  }

  #[inline]
  pub(crate) const fn expected(&self) -> &'static [&'static str] {
    match self {
      Self::Legacy(_) => TransactionLegacy::FIELDS,
      Self::Eip2930(_) => TransactionEip2930::FIELDS,
      Self::Eip1559(_) => TransactionEip1559::FIELDS,
      Self::Eip4844(_) => TransactionEip4844::FIELDS,
      Self::Eip7702(_) => TransactionEip7702::FIELDS,
    }
  }
}

impl Default for TransactionPayload {
  #[inline]
  fn default() -> Self {
    Self::Legacy(TransactionLegacy::default())
  }
}

impl Debug for TransactionPayload {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Legacy(inner) => Debug::fmt(inner, f),
      Self::Eip2930(inner) => Debug::fmt(inner, f),
      Self::Eip1559(inner) => Debug::fmt(inner, f),
      Self::Eip4844(inner) => Debug::fmt(inner, f),
      Self::Eip7702(inner) => Debug::fmt(inner, f),
    }
  }
}

impl From<TransactionLegacy> for TransactionPayload {
  #[inline]
  fn from(other: TransactionLegacy) -> Self {
    Self::Legacy(other)
  }
}

impl From<TransactionEip2930> for TransactionPayload {
  #[inline]
  fn from(other: TransactionEip2930) -> Self {
    Self::Eip2930(other)
  }
}

impl From<TransactionEip1559> for TransactionPayload {
  #[inline]
  fn from(other: TransactionEip1559) -> Self {
    Self::Eip1559(other)
  }
}

impl From<TransactionEip4844> for TransactionPayload {
  #[inline]
  fn from(other: TransactionEip4844) -> Self {
    Self::Eip4844(other)
  }
}

impl From<TransactionEip7702> for TransactionPayload {
  #[inline]
  fn from(other: TransactionEip7702) -> Self {
    Self::Eip7702(other)
  }
}

impl SerializeFlatten for TransactionPayload {
  const NAME: &'static str = "TransactionPayload";

  #[inline]
  fn fields(&self) -> usize {
    match self {
      Self::Legacy(inner) => inner.fields() + 1,
      Self::Eip2930(inner) => inner.fields() + 1,
      Self::Eip1559(inner) => inner.fields() + 1,
      Self::Eip4844(inner) => inner.fields() + 1,
      Self::Eip7702(inner) => inner.fields() + 1,
    }
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("type", &self.transaction_type())?;

    match self {
      Self::Legacy(inner) => inner.serialize_fields(state),
      Self::Eip2930(inner) => inner.serialize_fields(state),
      Self::Eip1559(inner) => inner.serialize_fields(state),
      Self::Eip4844(inner) => inner.serialize_fields(state),
      Self::Eip7702(inner) => inner.serialize_fields(state),
    }
  }
}

impl Serialize for TransactionPayload {
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

  use crate::rpc::TransactionPayload;

  #[test]
  fn test_serde() {
    let a: TransactionPayload = TransactionPayload::Legacy(Default::default());
    let b: TransactionPayload = TransactionPayload::Eip2930(Default::default());
    let c: TransactionPayload = TransactionPayload::Eip1559(Default::default());
    let d: TransactionPayload = TransactionPayload::Eip4844(Default::default());
    let e: TransactionPayload = TransactionPayload::Eip7702(Default::default());

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TransactionPayload",
        len: 8,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x0",
      },
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::None,
      Token::Str("nonce"),
      Token::Str("0x0"),
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("gasPrice"),
      Token::Str("0x0"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TransactionPayload",
        len: 10,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x1",
      },
      Token::Str("chainId"),
      Token::Str("0x0"),
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::None,
      Token::Str("nonce"),
      Token::Str("0x0"),
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("gasPrice"),
      Token::Str("0x0"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("accessList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    let tokens_c: &[Token] = &[
      Token::Struct {
        name: "TransactionPayload",
        len: 11,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x2",
      },
      Token::Str("chainId"),
      Token::Str("0x0"),
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::None,
      Token::Str("nonce"),
      Token::Str("0x0"),
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("maxPriorityFeePerGas"),
      Token::Str("0x0"),
      Token::Str("maxFeePerGas"),
      Token::Str("0x0"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("accessList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    let tokens_d: &[Token] = &[
      Token::Struct {
        name: "TransactionPayload",
        len: 13,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x3",
      },
      Token::Str("chainId"),
      Token::Str("0x0"),
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::None,
      Token::Str("nonce"),
      Token::Str("0x0"),
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("maxPriorityFeePerGas"),
      Token::Str("0x0"),
      Token::Str("maxFeePerGas"),
      Token::Str("0x0"),
      Token::Str("maxFeePerBlobGas"),
      Token::Str("0x0"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("accessList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("blobVersionedHashes"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    let tokens_e: &[Token] = &[
      Token::Struct {
        name: "TransactionPayload",
        len: 12,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x4",
      },
      Token::Str("chainId"),
      Token::Str("0x0"),
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::None,
      Token::Str("nonce"),
      Token::Str("0x0"),
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("maxPriorityFeePerGas"),
      Token::Str("0x0"),
      Token::Str("maxFeePerGas"),
      Token::Str("0x0"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("accessList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("authorizationList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
    assert_tokens(&c.readable(), tokens_c);
    assert_tokens(&d.readable(), tokens_d);
    assert_tokens(&e.readable(), tokens_e);
  }
}
