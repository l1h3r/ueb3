// Much of this code was 'borrowed' from the ethbloom crate.
//
// https://github.com/paritytech/parity-common/blob/a2b580d9fd5a340cea817bc9ed829320d2c9cd73/ethbloom/src/lib.rs
//
// See also: https://ethereum.github.io/yellowpaper/paper.pdf

use std::borrow::Cow;

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
  Data(&'a Slice),
  /// An owned hash of bytes.
  Hash(H256),
}

impl BloomInput<'_> {
  /// Converts `self` to a hash of bytes.
  #[inline]
  pub fn to_hash(&self) -> Cow<'_, H256> {
    match self {
      Self::Data(inner) => Cow::Owned(keccak256(inner)),
      Self::Hash(inner) => Cow::Borrowed(inner),
    }
  }
}

impl<'a> From<&'a Slice> for BloomInput<'a> {
  #[inline]
  fn from(other: &'a Slice) -> Self {
    Self::Data(other)
  }
}

impl<'a> From<&'a [u8]> for BloomInput<'a> {
  #[inline]
  fn from(other: &'a [u8]) -> Self {
    Self::Data(other.into())
  }
}

impl From<H256> for BloomInput<'_> {
  #[inline]
  fn from(other: H256) -> Self {
    Self::Hash(other)
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
  /// Returns `true` if `self` **may** contain the given `bloom`.
  ///
  /// Note: A notable property of bloom filter data structures is that they
  /// **may** return false positives but **will not** return false negatives.
  #[inline]
  pub fn contains_bloom(&self, other: &Self) -> bool {
    self & other == *other
  }

  /// Returns `true` if `self` **may** contain the given `input`.
  ///
  /// Note: A notable property of bloom filter data structures is that they
  /// **may** return false positives but **will not** return false negatives.
  #[inline]
  pub fn contains_input(&self, other: BloomInput<'_>) -> bool {
    self.contains_bloom(&other.into())
  }

  /// Updates `self` to include the data in the given `bloom`.
  #[inline]
  pub fn accrue_bloom(&mut self, bloom: &Self) {
    *self |= bloom;
  }

  /// Updates `self` to include the data in the given `input`.
  #[inline]
  pub fn accrue_input(&mut self, input: BloomInput<'_>) {
    const BLOOM_BYTES: usize = 256;
    const BLOOM_BITS: usize = BLOOM_BYTES * 8;
    const ITEM_BYTES: usize = BLOOM_BITS.ilog2().div_ceil(8) as usize;

    const MASK: usize = BLOOM_BITS - 1;

    let hash: Cow<'_, H256> = input.to_hash();
    let mut scope: usize = 0;

    for _ in 0..3 {
      let mut index: usize = 0;

      for _ in 0..ITEM_BYTES {
        index = (index << 8) | hash[scope] as usize;
        scope += 1;
      }

      index &= MASK;
      self[BLOOM_BYTES - 1 - index / 8] |= 1 << (index % 8);
    }
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::eth::Address;
  use crate::eth::Bloom;
  use crate::eth::BloomInput;
  use crate::eth::H256;

  #[test]
  fn test_accrue() {
    const BLOOM: &str = "0x00000004000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000080020000000000000200010000000000000000000001000000800000000000000000000000000000000000000000000000000000100100000000000000000000008000000000000000000000000000000002000000000000000000000";

    const ADDRESS: &str = "0xfd584430cafa2f451b4e2ebcf3986a21fff04350";
    const TOPIC_0: &str = "0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d";
    const TOPIC_1: &str = "0x4be29e0e4eb91f98f709d98803cba271592782e293b84a625e025cbb40197ba8";
    const TOPIC_2: &str = "0x000000000000000000000000835281a2563db4ebf1b626172e085dc406bfc7d2";
    const TOPIC_3: &str = "0x00000000000000000000000022896bfc68814bfd855b1a167255ee497006e730";

    let address: Address = ADDRESS.parse().unwrap();
    let topic_0: H256 = TOPIC_0.parse().unwrap();
    let topic_1: H256 = TOPIC_1.parse().unwrap();
    let topic_2: H256 = TOPIC_2.parse().unwrap();
    let topic_3: H256 = TOPIC_3.parse().unwrap();

    let input_a: BloomInput<'_> = BloomInput::Data(address.as_ref());
    let input_b: BloomInput<'_> = BloomInput::Data(topic_0.as_ref());
    let input_c: BloomInput<'_> = BloomInput::Data(topic_1.as_ref());
    let input_d: BloomInput<'_> = BloomInput::Data(topic_2.as_ref());
    let input_e: BloomInput<'_> = BloomInput::Data(topic_3.as_ref());

    let mut bloom: Bloom = Bloom::null();

    assert_eq!(bloom.contains_input(input_a), false);
    assert_eq!(bloom.contains_input(input_b), false);
    assert_eq!(bloom.contains_input(input_c), false);
    assert_eq!(bloom.contains_input(input_d), false);
    assert_eq!(bloom.contains_input(input_e), false);

    bloom.accrue_input(input_a);
    bloom.accrue_input(input_b);
    bloom.accrue_input(input_c);
    bloom.accrue_input(input_d);
    bloom.accrue_input(input_e);

    assert_eq!(bloom.contains_input(input_a), true);
    assert_eq!(bloom.contains_input(input_b), true);
    assert_eq!(bloom.contains_input(input_c), true);
    assert_eq!(bloom.contains_input(input_d), true);
    assert_eq!(bloom.contains_input(input_e), true);

    assert_eq!(bloom.to_string(), BLOOM);
  }
}
