use std::ops::RangeFrom;
use std::ops::RangeInclusive;
use std::ops::RangeToInclusive;

use crate::eth::U64;
use crate::rpc::BlockNumber;

// -----------------------------------------------------------------------------
// Ethereum Filter Block Range
// -----------------------------------------------------------------------------

/// A range of block numbers bounded inclusively below and above.
///
/// This type is constructed through a variety of [`From`] implementations:
///
///  * `From<u64|U64|BlockNumber>`matches an exact block by number.
///  * `From<RangeFrom<T>>` is bounded inclusively below `T` (`T..=BlockNumber::Latest`).
///  * `From<RangeInclusive<T>>` is bounded inclusively below and above `T` (`T..=T`).
///  * `From<RangeToInclusive<T>>` is bounded inclusively above `T` (`BlockNumber::Earliest..=T`).
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct FilterRange {
  pub(crate) src: BlockNumber,
  pub(crate) dst: BlockNumber,
}

impl FilterRange {
  /// Converts `self` into `(lower bound, upper (inclusive) bound)`.
  #[inline]
  pub const fn into_inner(self) -> (BlockNumber, BlockNumber) {
    (self.src, self.dst)
  }
}

impl From<u64> for FilterRange {
  #[inline]
  fn from(other: u64) -> Self {
    BlockNumber::from(other).into()
  }
}

impl From<U64> for FilterRange {
  #[inline]
  fn from(other: U64) -> Self {
    BlockNumber::from(other).into()
  }
}

impl From<BlockNumber> for FilterRange {
  #[inline]
  fn from(other: BlockNumber) -> Self {
    Self {
      src: other,
      dst: other,
    }
  }
}

impl<T> From<RangeFrom<T>> for FilterRange
where
  T: Into<BlockNumber>,
{
  #[inline]
  fn from(other: RangeFrom<T>) -> Self {
    Self {
      src: other.start.into(),
      dst: BlockNumber::Latest,
    }
  }
}

impl<T> From<RangeInclusive<T>> for FilterRange
where
  T: Into<BlockNumber>,
{
  #[inline]
  fn from(other: RangeInclusive<T>) -> Self {
    let (src, dst): (T, T) = other.into_inner();

    Self {
      src: src.into(),
      dst: dst.into(),
    }
  }
}

impl<T> From<RangeToInclusive<T>> for FilterRange
where
  T: Into<BlockNumber>,
{
  #[inline]
  fn from(other: RangeToInclusive<T>) -> Self {
    Self {
      src: BlockNumber::Earliest,
      dst: other.end.into(),
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::eth::U64;
  use crate::rpc::BlockNumber;
  use crate::rpc::FilterRange;

  #[test]
  fn test_default() {
    let range: FilterRange = FilterRange::default();
    assert_eq!(range.src, BlockNumber::Latest);
    assert_eq!(range.dst, BlockNumber::Latest);
  }

  #[test]
  fn test_from_number() {
    let range: FilterRange = 1.into();
    assert_eq!(range.src, BlockNumber::Integer(1));
    assert_eq!(range.dst, BlockNumber::Integer(1));

    let range: FilterRange = U64::from_u64(1).into();
    assert_eq!(range.src, BlockNumber::Integer(1));
    assert_eq!(range.dst, BlockNumber::Integer(1));

    let range: FilterRange = BlockNumber::Pending.into();
    assert_eq!(range.src, BlockNumber::Pending);
    assert_eq!(range.dst, BlockNumber::Pending);
  }

  #[test]
  fn test_from_range() {
    let range: FilterRange = (1..).into();
    assert_eq!(range.src, BlockNumber::Integer(1));
    assert_eq!(range.dst, BlockNumber::Latest);

    let range: FilterRange = (1..=10).into();
    assert_eq!(range.src, BlockNumber::Integer(1));
    assert_eq!(range.dst, BlockNumber::Integer(10));

    let range: FilterRange = (..=10).into();
    assert_eq!(range.src, BlockNumber::Earliest);
    assert_eq!(range.dst, BlockNumber::Integer(10));
  }
}
