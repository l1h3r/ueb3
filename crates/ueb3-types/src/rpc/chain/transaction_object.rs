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
use crate::eth::H256;
use crate::eth::Slice;
use crate::eth::U64;
use crate::eth::U256;
use crate::eth::Wei;
use crate::eth::YParity;

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
// Ethereum Transaction Object
// -----------------------------------------------------------------------------

/// A set of type-specific transaction properties.
#[derive(Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, tag = "type")]
pub enum TransactionObject {
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

impl TransactionObject {
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

  /// Returns the parity (0 for even, 1 for odd) of the y-value of the secp256k1 signature.
  #[inline]
  pub const fn y_parity(&self) -> Option<YParity> {
    match self {
      Self::Legacy(_) => None,
      Self::Eip2930(inner) => Some(inner.y_parity),
      Self::Eip1559(inner) => Some(inner.y_parity),
      Self::Eip4844(inner) => Some(inner.y_parity),
      Self::Eip7702(inner) => Some(inner.y_parity),
    }
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
  pub fn blob_versioned_hashes(&self) -> &[H256] {
    let Self::Eip4844(inner) = self else {
      return &[];
    };

    inner.blob_versioned_hashes.as_slice()
  }

  /// Returns a list of authorizations for the transaction.
  #[inline]
  pub const fn authorization_list(&self) -> Option<&AuthorizationList> {
    let Self::Eip7702(inner) = self else {
      return None;
    };

    Some(&inner.authorization_list)
  }
}

impl Default for TransactionObject {
  #[inline]
  fn default() -> Self {
    Self::Legacy(TransactionLegacy::default())
  }
}

impl Debug for TransactionObject {
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

impl From<TransactionLegacy> for TransactionObject {
  #[inline]
  fn from(other: TransactionLegacy) -> Self {
    Self::Legacy(other)
  }
}

impl From<TransactionEip2930> for TransactionObject {
  #[inline]
  fn from(other: TransactionEip2930) -> Self {
    Self::Eip2930(other)
  }
}

impl From<TransactionEip1559> for TransactionObject {
  #[inline]
  fn from(other: TransactionEip1559) -> Self {
    Self::Eip1559(other)
  }
}

impl From<TransactionEip4844> for TransactionObject {
  #[inline]
  fn from(other: TransactionEip4844) -> Self {
    Self::Eip4844(other)
  }
}

impl From<TransactionEip7702> for TransactionObject {
  #[inline]
  fn from(other: TransactionEip7702) -> Self {
    Self::Eip7702(other)
  }
}

impl SerializeFlatten for TransactionObject {
  const NAME: &'static str = "TransactionObject";

  #[inline]
  fn fields(&self) -> usize {
    match self {
      Self::Legacy(inner) => inner.fields(),
      Self::Eip2930(inner) => inner.fields(),
      Self::Eip1559(inner) => inner.fields(),
      Self::Eip4844(inner) => inner.fields(),
      Self::Eip7702(inner) => inner.fields(),
    }
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    match self {
      Self::Legacy(inner) => inner.serialize_fields(state),
      Self::Eip2930(inner) => inner.serialize_fields(state),
      Self::Eip1559(inner) => inner.serialize_fields(state),
      Self::Eip4844(inner) => inner.serialize_fields(state),
      Self::Eip7702(inner) => inner.serialize_fields(state),
    }
  }
}

impl Serialize for TransactionObject {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.serialize_struct(serializer)
  }
}

// -----------------------------------------------------------------------------
// Ethereum Transaction Object (legacy)
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
    8 + usize::from(self.chain_id.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("type", &Self::TYPE)?;

    if let Some(ref value) = self.chain_id.map(U64::from_u64) {
      state.serialize_field("chainId", value)?;
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
// Ethereum Transaction Object (eip-2930)
// -----------------------------------------------------------------------------

/// A transaction object (eip-2930).
#[derive(Clone, Hash, PartialEq, Eq, Deserialize)]
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
  /// The parity (0 for even, 1 for odd) of the y-value of the secp256k1 signature.
  pub y_parity: YParity,
  /// A list of addresses/storage keys that the transaction plans to access.
  pub access_list: AccessList,
}

impl TransactionEip2930 {
  /// The [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  pub const TYPE: TransactionType = TransactionType::Eip2930;
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
      .field("y_parity", &self.y_parity)
      .field("access_list", &self.access_list)
      .finish()
  }
}

impl SerializeFlatten for TransactionEip2930 {
  const NAME: &'static str = "TransactionEip2930";

  #[inline]
  fn fields(&self) -> usize {
    11
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("type", &Self::TYPE)?;
    state.serialize_field("chainId", &U64::from_u64(self.chain_id))?;
    state.serialize_field("from", &self.from)?;
    state.serialize_field("to", &self.to)?;
    state.serialize_field("nonce", &U64::from_u64(self.nonce))?;
    state.serialize_field("gas", &self.gas_limit)?;
    state.serialize_field("gasPrice", &self.gas_price)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;
    state.serialize_field("yParity", &self.y_parity)?;
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
// Ethereum Transaction Object (eip-1559)
// -----------------------------------------------------------------------------

/// A transaction object (eip-1559).
#[derive(Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionEip1559 {
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
  /// The parity (0 for even, 1 for odd) of the y-value of the secp256k1 signature.
  pub y_parity: YParity,
  /// A list of addresses/storage keys that the transaction plans to access.
  pub access_list: AccessList,
}

impl TransactionEip1559 {
  /// The [EIP-2718] transaction type identifier.
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  pub const TYPE: TransactionType = TransactionType::Eip1559;
}

impl Debug for TransactionEip1559 {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionEip1559")
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
      .field("y_parity", &self.y_parity)
      .field("access_list", &self.access_list)
      .finish()
  }
}

impl SerializeFlatten for TransactionEip1559 {
  const NAME: &'static str = "TransactionEip1559";

  #[inline]
  fn fields(&self) -> usize {
    12 + usize::from(self.gas_price.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("type", &Self::TYPE)?;
    state.serialize_field("chainId", &U64::from_u64(self.chain_id))?;
    state.serialize_field("from", &self.from)?;
    state.serialize_field("to", &self.to)?;
    state.serialize_field("nonce", &U64::from_u64(self.nonce))?;
    state.serialize_field("gas", &self.gas_limit)?;

    if let Some(ref value) = self.gas_price {
      state.serialize_field("gasPrice", value)?;
    } else {
      state.skip_field("gasPrice")?;
    }

    state.serialize_field("maxPriorityFeePerGas", &self.max_priority_fee_per_gas)?;
    state.serialize_field("maxFeePerGas", &self.max_fee_per_gas)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;
    state.serialize_field("yParity", &self.y_parity)?;
    state.serialize_field("accessList", &self.access_list)?;

    Ok(())
  }
}

impl Serialize for TransactionEip1559 {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.serialize_struct(serializer)
  }
}

// -----------------------------------------------------------------------------
// Ethereum Transaction Object (eip-4844)
// -----------------------------------------------------------------------------

/// A transaction object (eip-4844).
#[derive(Clone, Hash, PartialEq, Eq, Deserialize)]
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
  /// The parity (0 for even, 1 for odd) of the y-value of the secp256k1 signature.
  pub y_parity: YParity,
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
      .field("y_parity", &self.y_parity)
      .field("access_list", &self.access_list)
      .field("blob_versioned_hashes", &self.blob_versioned_hashes)
      .finish()
  }
}

impl SerializeFlatten for TransactionEip4844 {
  const NAME: &'static str = "TransactionEip4844";

  #[inline]
  fn fields(&self) -> usize {
    14 + usize::from(self.gas_price.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("type", &Self::TYPE)?;
    state.serialize_field("chainId", &U64::from_u64(self.chain_id))?;
    state.serialize_field("from", &self.from)?;
    state.serialize_field("to", &self.to)?;
    state.serialize_field("nonce", &U64::from_u64(self.nonce))?;
    state.serialize_field("gas", &self.gas_limit)?;

    if let Some(ref value) = self.gas_price {
      state.serialize_field("gasPrice", value)?;
    } else {
      state.skip_field("gasPrice")?;
    }

    state.serialize_field("maxPriorityFeePerGas", &self.max_priority_fee_per_gas)?;
    state.serialize_field("maxFeePerGas", &self.max_fee_per_gas)?;
    state.serialize_field("maxFeePerBlobGas", &self.max_fee_per_blob_gas)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;
    state.serialize_field("yParity", &self.y_parity)?;
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
// Ethereum Transaction Object (eip-7702)
// -----------------------------------------------------------------------------

/// A transaction object (eip-7702).
#[derive(Clone, Hash, PartialEq, Eq, Deserialize)]
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
  /// The parity (0 for even, 1 for odd) of the y-value of the secp256k1 signature.
  pub y_parity: YParity,
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
      .field("y_parity", &self.y_parity)
      .field("access_list", &self.access_list)
      .field("authorization_list", &self.authorization_list)
      .finish()
  }
}

impl SerializeFlatten for TransactionEip7702 {
  const NAME: &'static str = "TransactionEip7702";

  #[inline]
  fn fields(&self) -> usize {
    13 + usize::from(self.gas_price.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("type", &Self::TYPE)?;
    state.serialize_field("chainId", &U64::from_u64(self.chain_id))?;
    state.serialize_field("from", &self.from)?;
    state.serialize_field("to", &self.to)?;
    state.serialize_field("nonce", &U64::from_u64(self.nonce))?;
    state.serialize_field("gas", &self.gas_limit)?;

    if let Some(ref value) = self.gas_price {
      state.serialize_field("gasPrice", value)?;
    } else {
      state.skip_field("gasPrice")?;
    }

    state.serialize_field("maxPriorityFeePerGas", &self.max_priority_fee_per_gas)?;
    state.serialize_field("maxFeePerGas", &self.max_fee_per_gas)?;
    state.serialize_field("value", &self.value)?;
    state.serialize_field("input", &self.input)?;
    state.serialize_field("yParity", &self.y_parity)?;
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
