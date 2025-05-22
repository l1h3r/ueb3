use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;
use ueb3_utils::base16::FromHex;
use ueb3_utils::base16::FromHexError;

use crate::eip::eip1898::BlockHash;
use crate::eip::eip1898::BlockId;
use crate::eth::H256;
use crate::eth::U64;
use crate::rpc::BlockNumber;

// -----------------------------------------------------------------------------
// Ethereum Transaction Identifier
// -----------------------------------------------------------------------------

/// A transaction hash or block id/index pair.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum TransactionId {
  /// Transaction hash.
  Hash(H256),
  /// Block id/transaction index pair.
  Block(BlockId, u64),
}

impl TransactionId {
  /// Returns `true` if `self` is a hash referencing a specific transaction.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::TransactionId;
  ///
  /// assert_eq!(TransactionId::Hash(H256::NULL).is_hash(), true);
  /// assert_eq!(TransactionId::Block(H256::NULL.into(), 0).is_hash(), false);
  /// ```
  #[inline]
  pub const fn is_hash(&self) -> bool {
    matches!(self, Self::Hash(_))
  }

  /// Returns `true` if `self` is a block id/index pair.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::TransactionId;
  ///
  /// assert_eq!(TransactionId::Block(H256::NULL.into(), 0).is_block(), true);
  /// assert_eq!(TransactionId::Hash(H256::NULL).is_block(), false);
  /// ```
  #[inline]
  pub const fn is_block(&self) -> bool {
    matches!(self, Self::Block(_, _))
  }

  /// Returns a reference to the block hash.
  ///
  /// This may return `None` if the transaction id is a block id/index variant.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::TransactionId;
  ///
  /// assert_eq!(TransactionId::Hash(H256::NULL).hash(), Some(&H256::NULL));
  /// assert_eq!(TransactionId::Block(H256::NULL.into(), 0).hash(), None);
  /// ```
  #[inline]
  pub const fn hash(&self) -> Option<&H256> {
    match self {
      Self::Hash(inner) => Some(inner),
      Self::Block(_, _) => None,
    }
  }

  /// Converts `self` to a transaction hash.
  ///
  /// This may return `None` if the transaction id is a block id/index variant.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::TransactionId;
  ///
  /// assert_eq!(TransactionId::Hash(H256::NULL).to_hash(), Some(H256::NULL));
  /// assert_eq!(TransactionId::Block(H256::NULL.into(), 0).to_hash(), None);
  /// ```
  #[inline]
  pub const fn to_hash(&self) -> Option<H256> {
    match self {
      Self::Hash(inner) => Some(*inner),
      Self::Block(_, _) => None,
    }
  }
}

impl Default for TransactionId {
  #[inline]
  fn default() -> Self {
    Self::Block(BlockId::default(), 0)
  }
}

impl Debug for TransactionId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Hash(value) => write!(f, "Hash({value})"),
      Self::Block(block, index) => write!(f, "Block({block}, {index})"),
    }
  }
}

impl Display for TransactionId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Hash(value) => write!(f, "{value}"),
      Self::Block(block, index) => write!(f, "{block} - {index}"),
    }
  }
}

impl From<H256> for TransactionId {
  #[inline]
  fn from(other: H256) -> Self {
    Self::Hash(other)
  }
}

impl From<(u64, u64)> for TransactionId {
  #[inline]
  fn from((block, index): (u64, u64)) -> Self {
    Self::Block(block.into(), index)
  }
}

impl From<(u64, U64)> for TransactionId {
  #[inline]
  fn from((block, index): (u64, U64)) -> Self {
    Self::Block(block.into(), index.into_u64())
  }
}

impl From<(U64, u64)> for TransactionId {
  #[inline]
  fn from((block, index): (U64, u64)) -> Self {
    Self::Block(block.into(), index)
  }
}

impl From<(U64, U64)> for TransactionId {
  #[inline]
  fn from((block, index): (U64, U64)) -> Self {
    Self::Block(block.into(), index.into_u64())
  }
}

impl From<(BlockNumber, u64)> for TransactionId {
  #[inline]
  fn from((block, index): (BlockNumber, u64)) -> Self {
    Self::Block(block.into(), index)
  }
}

impl From<(BlockNumber, U64)> for TransactionId {
  #[inline]
  fn from((block, index): (BlockNumber, U64)) -> Self {
    Self::Block(block.into(), index.into_u64())
  }
}

impl From<(H256, u64)> for TransactionId {
  #[inline]
  fn from((block, index): (H256, u64)) -> Self {
    Self::Block(block.into(), index)
  }
}

impl From<(H256, U64)> for TransactionId {
  #[inline]
  fn from((block, index): (H256, U64)) -> Self {
    Self::Block(block.into(), index.into_u64())
  }
}

impl From<(BlockHash, u64)> for TransactionId {
  #[inline]
  fn from((block, index): (BlockHash, u64)) -> Self {
    Self::Block(block.into(), index)
  }
}

impl From<(BlockHash, U64)> for TransactionId {
  #[inline]
  fn from((block, index): (BlockHash, U64)) -> Self {
    Self::Block(block.into(), index.into_u64())
  }
}

impl From<(BlockId, u64)> for TransactionId {
  #[inline]
  fn from((block, index): (BlockId, u64)) -> Self {
    Self::Block(block, index)
  }
}

impl From<(BlockId, U64)> for TransactionId {
  #[inline]
  fn from((block, index): (BlockId, U64)) -> Self {
    Self::Block(block, index.into_u64())
  }
}

impl FromStr for TransactionId {
  type Err = FromHexError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    FromHex::from_str(input)
  }
}

impl FromHex for TransactionId {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::Hash)
  }
}
