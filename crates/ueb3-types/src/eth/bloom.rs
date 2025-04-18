// Much of this code was 'borrowed' from the ethbloom crate.
//
// https://github.com/paritytech/parity-common/blob/a2b580d9fd5a340cea817bc9ed829320d2c9cd73/ethbloom/src/lib.rs
//
// See also: https://ethereum.github.io/yellowpaper/paper.pdf

use crate::crypto::keccak256;
use crate::eth::H256;
use crate::eth::Slice;

// ------------—------------—------------—------------—------------—-------
// Bloom Filter Input
// ------------—------------—------------—------------—------------—-------

/// A bloom filter input.
#[derive(Clone, Copy, Debug)]
pub enum BloomInput<'a> {
  /// A slice of unhashed bytes.
  Slice(&'a Slice),
  /// An owned hash of bytes.
  Bytes(H256),
}

impl BloomInput<'_> {
  /// Converts `self` to a hash of bytes.
  #[inline]
  pub fn to_hash(&self) -> H256 {
    match self {
      Self::Slice(inner) => keccak256(inner.as_slice()),
      Self::Bytes(inner) => *inner,
    }
  }
}

impl<'a> From<&'a [u8]> for BloomInput<'a> {
  #[inline]
  fn from(other: &'a [u8]) -> Self {
    Self::Slice(Slice::new(other))
  }
}

impl<'a> From<&'a Slice> for BloomInput<'a> {
  #[inline]
  fn from(other: &'a Slice) -> Self {
    Self::Slice(other)
  }
}

impl From<H256> for BloomInput<'_> {
  #[inline]
  fn from(other: H256) -> Self {
    Self::Bytes(other)
  }
}

// ------------—------------—------------—------------—------------—-------
// Bloom Filter
// ------------—------------—------------—------------—------------—-------

define_hash!(
  Bloom(256),
  "An Ethereum bloom filter as a fixed-size array of 256 bytes"
);

impl Bloom {
  /// Updates `self` to include the data in the given `bloom`.
  #[inline]
  pub fn accrue_bloom(&mut self, bloom: &Self) {
    *self |= bloom;
  }

  /// Updates `self` to include the data in the given `input`.
  #[inline]
  pub fn accrue_input(&mut self, input: BloomInput<'_>) {
    const DATA_BYTES: usize = 256;
    const DATA_BITS: usize = DATA_BYTES * 8;

    const ITEM_BYTES: usize = DATA_BITS.ilog2().div_ceil(8) as usize;
    const ITEM_BITS: usize = 3;

    const MASK: usize = DATA_BITS - 1;

    let hash: H256 = input.to_hash();
    let mut scope: usize = 0;

    for _ in 0..ITEM_BITS {
      let mut index: usize = 0;

      for _ in 0..ITEM_BYTES {
        index = (index << 8) | hash[scope] as usize;
        scope += 1;
      }

      index &= MASK;
      self[DATA_BYTES - 1 - index / 8] |= 1 << (index % 8);
    }
  }

  /// Returns `true` if `self` **may** contain the given `bloom`.
  ///
  /// Note: A notable property of bloom filter data structures is that they
  /// **may** return false positives but **will not** return false negatives.
  #[inline]
  pub fn contains_bloom(&self, bloom: &Self) -> bool {
    self & bloom == *bloom
  }

  /// Returns `true` if `self` **may** contain the given `input`.
  ///
  /// Note: A notable property of bloom filter data structures is that they
  /// **may** return false positives but **will not** return false negatives.
  #[inline]
  pub fn contains_input(&self, input: BloomInput<'_>) -> bool {
    self.contains_bloom(&input.into())
  }
}

impl From<BloomInput<'_>> for Bloom {
  #[inline]
  fn from(other: BloomInput<'_>) -> Self {
    let mut this: Self = Self::null();
    this.accrue_input(other);
    this
  }
}
