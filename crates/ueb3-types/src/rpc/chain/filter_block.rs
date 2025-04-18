use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::RangeFrom;
use std::ops::RangeInclusive;
use std::ops::RangeToInclusive;

use crate::eth::H256;
use crate::eth::U64;
use crate::rpc::BlockNumber;
use crate::rpc::FilterRange;

// -----------------------------------------------------------------------------
// Ethereum Filter Block Options
// -----------------------------------------------------------------------------

/// A log filter block hash or range of blocks.
///
/// Block hash matches an exact block as specified by [EIP-234].
///
/// See [`FilterRange`] for more information on how to construct ranges.
///
/// [EIP-234]: https://eips.ethereum.org/EIPS/eip-234
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum FilterBlock {
  /// A block hash identifier.
  Exact(H256),
  /// An inclusive range of blocks.
  Range(FilterRange),
  /// An empty block specifier.
  #[default]
  Empty,
}

impl FilterBlock {
  /// Returns `true` if `self` is a hash referencing a specific block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::FilterBlock;
  ///
  /// assert_eq!(FilterBlock::from(H256::NULL).is_exact(), true);
  /// assert_eq!(FilterBlock::from(0).is_exact(), false);
  /// assert_eq!(FilterBlock::Empty.is_exact(), false);
  /// ```
  #[inline]
  pub const fn is_exact(&self) -> bool {
    matches!(self, Self::Exact(_))
  }

  /// Returns `true` if `self` is an inclusive range of blocks.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::eth::U64;
  /// use ueb3_types::rpc::BlockNumber;
  /// use ueb3_types::rpc::FilterBlock;
  ///
  /// assert_eq!(FilterBlock::from(0).is_range(), true);
  /// assert_eq!(FilterBlock::from(U64::from_u64(0)).is_range(), true);
  /// assert_eq!(FilterBlock::from(BlockNumber::Pending).is_range(), true);
  /// assert_eq!(FilterBlock::from(1..).is_range(), true);
  /// assert_eq!(FilterBlock::from(1..=10).is_range(), true);
  /// assert_eq!(FilterBlock::from(..=10).is_range(), true);
  /// assert_eq!(FilterBlock::from(H256::NULL).is_range(), false);
  /// assert_eq!(FilterBlock::Empty.is_range(), false);
  /// ```
  #[inline]
  pub const fn is_range(&self) -> bool {
    matches!(self, Self::Range(_))
  }

  /// Returns `true` if `self` is an empty block specifier.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::FilterBlock;
  ///
  /// assert_eq!(FilterBlock::from(H256::NULL).is_empty(), false);
  /// assert_eq!(FilterBlock::from(0).is_empty(), false);
  /// assert_eq!(FilterBlock::Empty.is_empty(), true);
  /// ```
  #[inline]
  pub const fn is_empty(&self) -> bool {
    matches!(self, Self::Empty)
  }

  /// Returns a reference to the block hash
  ///
  /// This may return `None` if the block filter is a range.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::FilterBlock;
  /// use ueb3_types::rpc::FilterRange;
  ///
  /// let exact: H256 = H256::NULL;
  /// let range: FilterRange = (1..).into();
  ///
  /// assert_eq!(FilterBlock::from(exact).hash(), Some(&exact));
  /// assert_eq!(FilterBlock::from(range).hash(), None);
  /// ```
  #[inline]
  pub const fn hash(&self) -> Option<&H256> {
    match self {
      Self::Exact(inner) => Some(inner),
      Self::Range(_) => None,
      Self::Empty => None,
    }
  }

  /// Returns a reference to the range of blocks.
  ///
  /// This may return `None` if the block filter is a hash.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::FilterBlock;
  /// use ueb3_types::rpc::FilterRange;
  ///
  /// let exact: H256 = H256::NULL;
  /// let range: FilterRange = (1..).into();
  ///
  /// assert_eq!(FilterBlock::from(exact).range(), None);
  /// assert_eq!(FilterBlock::from(range).range(), Some(&range));
  /// ```
  #[inline]
  pub const fn range(&self) -> Option<&FilterRange> {
    match self {
      Self::Exact(_) => None,
      Self::Range(inner) => Some(inner),
      Self::Empty => None,
    }
  }

  /// Returns the block hash.
  ///
  /// This may return `None` if the block filter is a range.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::FilterBlock;
  /// use ueb3_types::rpc::FilterRange;
  ///
  /// let exact: H256 = H256::NULL;
  /// let range: FilterRange = (1..).into();
  ///
  /// assert_eq!(FilterBlock::from(exact).to_hash(), Some(exact));
  /// assert_eq!(FilterBlock::from(range).to_hash(), None);
  /// ```
  #[inline]
  pub const fn to_hash(&self) -> Option<H256> {
    self.hash().copied()
  }

  /// Returns the block range.
  ///
  /// This may return `None` if the block filter is a hash.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::FilterBlock;
  /// use ueb3_types::rpc::FilterRange;
  ///
  /// let exact: H256 = H256::NULL;
  /// let range: FilterRange = (1..).into();
  ///
  /// assert_eq!(FilterBlock::from(exact).to_range(), None);
  /// assert_eq!(FilterBlock::from(range).to_range(), Some(range));
  /// ```
  #[inline]
  pub const fn to_range(&self) -> Option<FilterRange> {
    self.range().copied()
  }
}

impl Debug for FilterBlock {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Exact(inner) => Debug::fmt(inner, f),
      Self::Range(inner) => Debug::fmt(inner, f),
      Self::Empty => f.write_str("Empty"),
    }
  }
}

impl From<H256> for FilterBlock {
  #[inline]
  fn from(other: H256) -> Self {
    Self::Exact(other)
  }
}

impl From<u64> for FilterBlock {
  #[inline]
  fn from(other: u64) -> Self {
    Self::Range(other.into())
  }
}

impl From<U64> for FilterBlock {
  #[inline]
  fn from(other: U64) -> Self {
    Self::Range(other.into())
  }
}

impl From<BlockNumber> for FilterBlock {
  #[inline]
  fn from(other: BlockNumber) -> Self {
    Self::Range(other.into())
  }
}

impl<T> From<RangeFrom<T>> for FilterBlock
where
  T: Into<BlockNumber>,
{
  #[inline]
  fn from(other: RangeFrom<T>) -> Self {
    Self::Range(other.into())
  }
}

impl<T> From<RangeInclusive<T>> for FilterBlock
where
  T: Into<BlockNumber>,
{
  #[inline]
  fn from(other: RangeInclusive<T>) -> Self {
    Self::Range(other.into())
  }
}

impl<T> From<RangeToInclusive<T>> for FilterBlock
where
  T: Into<BlockNumber>,
{
  #[inline]
  fn from(other: RangeToInclusive<T>) -> Self {
    Self::Range(other.into())
  }
}

impl From<FilterRange> for FilterBlock {
  #[inline]
  fn from(other: FilterRange) -> Self {
    Self::Range(other)
  }
}
