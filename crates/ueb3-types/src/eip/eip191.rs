//! Extra types defined in [EIP-191].
//!
//! [EIP-191]: https://eips.ethereum.org/EIPS/eip-191

use itoa::Buffer;

use crate::crypto::keccak256;
use crate::eth::Address;
use crate::eth::H256;
use crate::eth::Slice;

const PREFIX: u8 = 0x19;
const PREFIX_V45: &[u8] = b"Ethereum Signed Message:\n";

// -----------------------------------------------------------------------------
// Version
// -----------------------------------------------------------------------------

/// A version byte.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
  /// A version representing `Data with intended validator`.
  V00 = 0x00,
  /// A version representing `Structured data`.
  ///
  /// Note: This format is defined in [EIP-712].
  ///
  /// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712
  V01 = 0x01,
  /// A version representing `personal_sign messages`.
  V45 = 0x45,
}

impl Version {
  #[inline]
  fn new_v00<T>(address: &Address, input: &T) -> Vec<u8>
  where
    T: AsRef<Slice> + ?Sized,
  {
    fn inner(address: &Address, input: &Slice) -> Vec<u8> {
      let capacity: usize = 1 + 1 + size_of::<Address>() + input.len();
      let mut data: Vec<u8> = Vec::with_capacity(capacity);

      data.push(PREFIX);
      data.push(Version::V00 as u8);
      data.extend_from_slice(address.as_ref());
      data.extend_from_slice(input);
      data
    }

    inner(address, input.as_ref())
  }

  #[inline]
  fn new_v45<T>(input: &T) -> Vec<u8>
  where
    T: AsRef<Slice> + ?Sized,
  {
    fn inner(input: &Slice) -> Vec<u8> {
      let mut buffer: Buffer = Buffer::new();

      let data_len: usize = input.len();
      let size_str: &str = buffer.format(data_len);

      let capacity: usize = 1 + PREFIX_V45.len() + size_str.len() + data_len;
      let mut data: Vec<u8> = Vec::with_capacity(capacity);

      data.push(PREFIX);
      data.extend_from_slice(PREFIX_V45);
      data.extend_from_slice(size_str.as_bytes());
      data.extend_from_slice(input);
      data
    }

    inner(input.as_ref())
  }
}

// -----------------------------------------------------------------------------
// Utilities
// -----------------------------------------------------------------------------

/// Hash the given `input` according to `Version 0x00` of [EIP-191].
///
/// [EIP-191]: https://eips.ethereum.org/EIPS/eip-191
#[inline]
pub fn hash_0x00<T>(address: &Address, input: &T) -> H256
where
  T: AsRef<Slice> + ?Sized,
{
  keccak256(Version::new_v00(address, input).as_slice())
}

/// Hash the given `input` according to `Version 0x45` of [EIP-191].
///
/// [EIP-191]: https://eips.ethereum.org/EIPS/eip-191
#[inline]
pub fn hash_0x45<T>(input: &T) -> H256
where
  T: AsRef<Slice> + ?Sized,
{
  keccak256(Version::new_v45(input).as_slice())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip191::hash_0x45;

  #[test]
  #[ignore]
  fn test_serde_v00() {
    unimplemented!()
  }

  #[test]
  #[ignore]
  fn test_serde_v01() {
    unimplemented!()
  }

  #[test]
  fn test_serde_v45() {
    assert_tokens(
      &hash_0x45("Hello World").readable(),
      &[Token::Str(
        "0xa1de988600a42c4b4ab089b619297c17d53cffae5d5120d82d8a92d0bb3b78f2",
      )],
    );
  }
}
