//! Extra types defined in [EIP-1898].
//!
//! [EIP-1898]: https://eips.ethereum.org/EIPS/eip-1898

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use serde::de::MapAccess;
use serde::de::Unexpected;
use serde::de::Visitor;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Not;
use std::str::FromStr;
use ueb3_serde::maybe::Maybe;
use ueb3_utils::base16::FromHex;
use ueb3_utils::base16::FromHexError;

use crate::eth::H256;
use crate::eth::U64;
use crate::rpc::BlockNumber;

// 256-bit hash == 32 bytes == 0x + 64 hex chars == 66 chars
const HLEN: usize = 66;

// -----------------------------------------------------------------------------
// JSON-RPC Block Hash
// -----------------------------------------------------------------------------

/// A block hash identifier.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockHash {
  #[serde(rename = "blockHash")]
  hash: H256,
  #[serde(default, rename = "requireCanonical", skip_serializing_if = "Not::not")]
  canonical: bool,
}

impl BlockHash {
  /// Creates a new `BlockHash`.
  #[inline]
  pub const fn new(hash: H256, canonical: bool) -> Self {
    Self { hash, canonical }
  }

  /// Returns `true` if `self` requires the block to be a canonical block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockHash;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockHash::new(H256::NULL, true).is_canonical(), true);
  /// assert_eq!(BlockHash::new(H256::NULL, false).is_canonical(), false);
  /// ```
  #[inline]
  pub const fn is_canonical(&self) -> bool {
    self.canonical
  }

  /// Returns a reference to the block hash.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockHash;
  /// use ueb3_types::eth::H256;
  ///
  /// // Ethereum genesis block
  /// const HASH: &str = "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3";
  ///
  /// let hash: H256 = HASH.parse().unwrap();
  /// let data: BlockHash = BlockHash::new(hash, false);
  ///
  /// assert_eq!(data.hash(), &hash);
  /// ```
  #[inline]
  pub const fn hash(&self) -> &H256 {
    &self.hash
  }

  /// Converts `self` to a block hash.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockHash;
  /// use ueb3_types::eth::H256;
  ///
  /// // Ethereum genesis block
  /// const HASH: &str = "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3";
  ///
  /// let hash: H256 = HASH.parse().unwrap();
  /// let data: BlockHash = BlockHash::new(hash, false);
  ///
  /// assert_eq!(data.to_hash(), hash);
  /// ```
  #[inline]
  pub const fn to_hash(&self) -> H256 {
    self.hash
  }
}

impl Debug for BlockHash {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "BlockHash({}, {})", self.hash, self.canonical)
  }
}

impl Display for BlockHash {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if self.canonical {
      write!(f, "canonical block {}", self.hash)
    } else {
      write!(f, "block {}", self.hash)
    }
  }
}

impl From<H256> for BlockHash {
  #[inline]
  fn from(other: H256) -> Self {
    Self::new(other, false)
  }
}

impl From<(H256, bool)> for BlockHash {
  #[inline]
  fn from((hash, canonical): (H256, bool)) -> Self {
    Self::new(hash, canonical)
  }
}

impl AsRef<H256> for BlockHash {
  #[inline]
  fn as_ref(&self) -> &H256 {
    self.hash()
  }
}

impl FromStr for BlockHash {
  type Err = FromHexError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    FromHex::from_str(input)
  }
}

impl FromHex for BlockHash {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    H256::from_slice(input).map(Into::into)
  }
}

// -----------------------------------------------------------------------------
// JSON-RPC Block Identifier
// -----------------------------------------------------------------------------

/// A block number or block hash identifier.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum BlockId {
  /// Block number.
  Bnum(BlockNumber),
  /// Block hash.
  Hash(BlockHash),
}

impl BlockId {
  /// Creates a new `BlockId` referencing the earliest/genesis block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockId::earliest(), BlockNumber::Earliest.into());
  /// ```
  #[inline]
  pub const fn earliest() -> Self {
    Self::Bnum(BlockNumber::Earliest)
  }

  /// Creates a new `BlockId` referencing the latest proposed block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockId::latest(), BlockNumber::Latest.into());
  /// ```
  #[inline]
  pub const fn latest() -> Self {
    Self::Bnum(BlockNumber::Latest)
  }

  /// Creates a new `BlockId` referencing the latest safe head block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockId::safe(), BlockNumber::Safe.into());
  /// ```
  #[inline]
  pub const fn safe() -> Self {
    Self::Bnum(BlockNumber::Safe)
  }

  /// Creates a new `BlockId` referencing the latest finalized block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockId::finalized(), BlockNumber::Finalized.into());
  /// ```
  #[inline]
  pub const fn finalized() -> Self {
    Self::Bnum(BlockNumber::Finalized)
  }

  /// Creates a new `BlockId` referencing the pending state.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockId::pending(), BlockNumber::Pending.into());
  /// ```
  #[inline]
  pub const fn pending() -> Self {
    Self::Bnum(BlockNumber::Pending)
  }

  /// Creates a new `BlockId` as an integer referencing a specific block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::U64;
  ///
  /// assert_eq!(BlockId::integer(0), 0.into());
  /// assert_eq!(BlockId::integer(0), U64::from_u64(0).into());
  /// ```
  #[inline]
  pub const fn integer(value: u64) -> Self {
    Self::Bnum(BlockNumber::Integer(value))
  }

  /// Creates a new `BlockId` as a block hash referencing a specific block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// // Ethereum genesis block
  /// const HASH: &str = "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3";
  ///
  /// let hash: H256 = HASH.parse().unwrap();
  ///
  /// assert_eq!(BlockId::hash(hash, false), BlockId::from(hash));
  /// ```
  #[inline]
  pub const fn hash(hash: H256, canonical: bool) -> Self {
    Self::Hash(BlockHash::new(hash, canonical))
  }

  /// Returns `true` if `self` is a tag referencing the earliest/genesis block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::earliest().is_earliest(), true);
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_earliest(), false);
  /// ```
  #[inline]
  pub const fn is_earliest(&self) -> bool {
    matches!(self, Self::Bnum(BlockNumber::Earliest))
  }

  /// Returns `true` if `self` is a tag referencing the latest proposed block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::latest().is_latest(), true);
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_latest(), false);
  /// ```
  #[inline]
  pub const fn is_latest(&self) -> bool {
    matches!(self, Self::Bnum(BlockNumber::Latest))
  }

  /// Returns `true` if `self` is a tag referencing the latest safe head block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::safe().is_safe(), true);
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_safe(), false);
  /// ```
  #[inline]
  pub const fn is_safe(&self) -> bool {
    matches!(self, Self::Bnum(BlockNumber::Safe))
  }

  /// Returns `true` if `self` is a tag referencing the latest finalized block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::finalized().is_finalized(), true);
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_finalized(), false);
  /// ```
  #[inline]
  pub const fn is_finalized(&self) -> bool {
    matches!(self, Self::Bnum(BlockNumber::Finalized))
  }

  /// Returns `true` if `self` is a tag referencing the pending state.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::pending().is_pending(), true);
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_pending(), false);
  /// ```
  #[inline]
  pub const fn is_pending(&self) -> bool {
    matches!(self, Self::Bnum(BlockNumber::Pending))
  }

  /// Returns `true` if `self` is an integer referencing a specific block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::integer(0).is_integer(), true);
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_integer(), false);
  /// ```
  #[inline]
  pub const fn is_integer(&self) -> bool {
    matches!(self, Self::Bnum(BlockNumber::Integer(_)))
  }

  /// Returns `true` if `self` is a hash referencing a specific block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_hash(), true);
  /// ```
  #[inline]
  pub const fn is_hash(&self) -> bool {
    matches!(self, Self::Hash(_))
  }

  /// Returns `true` if `self` requires the block to be a canonical block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::hash(H256::NULL, true).is_canonical(), true);
  /// assert_eq!(BlockId::hash(H256::NULL, false).is_canonical(), false);
  /// ```
  #[inline]
  pub const fn is_canonical(&self) -> bool {
    let Self::Hash(inner) = self else {
      return false;
    };

    inner.is_canonical()
  }

  /// Converts `self` to a `BlockNumber`.
  ///
  /// This may return `None` if the block id is a hash or tag variant.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockId::earliest().to_number(), Some(BlockNumber::Earliest));
  /// assert_eq!(BlockId::latest().to_number(), Some(BlockNumber::Latest));
  /// assert_eq!(BlockId::safe().to_number(), Some(BlockNumber::Safe));
  /// assert_eq!(BlockId::finalized().to_number(), Some(BlockNumber::Finalized));
  /// assert_eq!(BlockId::pending().to_number(), Some(BlockNumber::Pending));
  /// assert_eq!(BlockId::integer(0).to_number(), Some(BlockNumber::Integer(0)));
  /// assert_eq!(BlockId::hash(H256::NULL, false).to_number(), None);
  /// ```
  #[inline]
  pub const fn to_number(&self) -> Option<BlockNumber> {
    match self {
      Self::Bnum(inner) => Some(*inner),
      Self::Hash(_) => None,
    }
  }

  /// Converts `self` to a 64-bit integer.
  ///
  /// This may return `None` if the block id is a hash or tag variant.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::earliest().to_integer(), None);
  /// assert_eq!(BlockId::latest().to_integer(), None);
  /// assert_eq!(BlockId::safe().to_integer(), None);
  /// assert_eq!(BlockId::finalized().to_integer(), None);
  /// assert_eq!(BlockId::pending().to_integer(), None);
  /// assert_eq!(BlockId::integer(0).to_integer(), Some(0));
  /// assert_eq!(BlockId::hash(H256::NULL, false).to_integer(), None);
  #[inline]
  pub const fn to_integer(&self) -> Option<u64> {
    match self {
      Self::Bnum(inner) => inner.to_integer(),
      Self::Hash(_) => None,
    }
  }

  /// Converts `self` to a block hash.
  ///
  /// This may return `None` if the block id is an integer or tag variant.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip1898::BlockId;
  /// use ueb3_types::eth::H256;
  ///
  /// assert_eq!(BlockId::earliest().to_hash(), None);
  /// assert_eq!(BlockId::latest().to_hash(), None);
  /// assert_eq!(BlockId::safe().to_hash(), None);
  /// assert_eq!(BlockId::finalized().to_hash(), None);
  /// assert_eq!(BlockId::pending().to_hash(), None);
  /// assert_eq!(BlockId::integer(0).to_hash(), None);
  /// assert_eq!(BlockId::hash(H256::NULL, false).to_hash(), Some(H256::NULL));
  #[inline]
  pub const fn to_hash(&self) -> Option<H256> {
    match self {
      Self::Bnum(_) => None,
      Self::Hash(inner) => Some(inner.to_hash()),
    }
  }
}

impl Default for BlockId {
  #[inline]
  fn default() -> Self {
    Self::Bnum(BlockNumber::default())
  }
}

impl Debug for BlockId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Bnum(inner) => Debug::fmt(inner, f),
      Self::Hash(inner) => Debug::fmt(inner, f),
    }
  }
}

impl Display for BlockId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Bnum(inner) => Display::fmt(inner, f),
      Self::Hash(inner) => Display::fmt(inner, f),
    }
  }
}

impl From<u64> for BlockId {
  #[inline]
  fn from(other: u64) -> Self {
    Self::Bnum(other.into())
  }
}

impl From<U64> for BlockId {
  #[inline]
  fn from(other: U64) -> Self {
    Self::Bnum(other.into())
  }
}

impl From<BlockNumber> for BlockId {
  #[inline]
  fn from(other: BlockNumber) -> Self {
    Self::Bnum(other)
  }
}

impl From<H256> for BlockId {
  #[inline]
  fn from(other: H256) -> Self {
    Self::Hash(other.into())
  }
}

impl From<(H256, bool)> for BlockId {
  #[inline]
  fn from(other: (H256, bool)) -> Self {
    Self::Hash(other.into())
  }
}

impl From<BlockHash> for BlockId {
  #[inline]
  fn from(other: BlockHash) -> Self {
    Self::Hash(other)
  }
}

impl FromStr for BlockId {
  type Err = FromHexError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    if input.len() == HLEN {
      input.parse().map(Self::Hash)
    } else {
      input.parse().map(Self::Bnum)
    }
  }
}

impl FromHex for BlockId {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    if input.len() == HLEN {
      FromHex::from_slice(input).map(Self::Hash)
    } else {
      FromHex::from_slice(input).map(Self::Bnum)
    }
  }
}

impl<'de> Deserialize<'de> for BlockId {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    const ERR_VARIANT_BOTH: &str = "expected one of `blockNumber` or `blockHash` (found both)";
    const ERR_VARIANT_NONE: &str = "expected one of `blockNumber` or `blockHash` (found none)";

    struct __Visitor;

    impl<'de> Visitor<'de> for __Visitor {
      type Value = BlockId;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("an EIP-1898 block identifer")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: Error,
      {
        value.parse().map_err(E::custom)
      }

      #[rustfmt::skip]
      fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        #[derive(Deserialize)]
        #[serde(variant_identifier)]
        enum Field {
          #[serde(rename = "blockNumber")]
          BlockNumber,
          #[serde(rename = "blockHash")]
          BlockHash,
          #[serde(rename = "requireCanonical")]
          Canonical,
        }

        let mut block_number: Maybe<A, BlockNumber> = Maybe::NONE;
        let mut block_hash: Maybe<A, H256> = Maybe::NONE;
        let mut require_canonical: Maybe<A, bool> = Maybe::NONE;

        while let Some(field) = access.next_key()? {
          match field {
            Field::BlockNumber => block_number.set(&mut access, "blockNumber")?,
            Field::BlockHash => block_hash.set(&mut access, "blockHash")?,
            Field::Canonical => require_canonical.set(&mut access, "requireCanonical")?,
          }
        }

        let block_number: Option<BlockNumber> = block_number.get_optional();
        let block_hash: Option<H256> = block_hash.get_optional();
        let require_canonical: Option<bool> = require_canonical.get_optional();

        match (block_number, block_hash, require_canonical) {
          (Some(value), None, None) => {
            Ok(BlockId::Bnum(value))
          }
          (None, Some(value), canonical) => {
            Ok(BlockId::Hash(BlockHash::new(value, canonical.unwrap_or_default())))
          }
          (Some(_), None, Some(canonical)) => {
            Err(A::Error::invalid_value(Unexpected::Bool(canonical), &self))
          }
          (Some(_), Some(_), _) => {
            Err(A::Error::custom(ERR_VARIANT_BOTH))
          }
          (None, None, _) => {
            Err(A::Error::custom(ERR_VARIANT_NONE))
          }
        }
      }
    }

    deserializer.deserialize_any(__Visitor)
  }
}

impl Serialize for BlockId {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Bnum(inner) => inner.serialize(serializer),
      Self::Hash(inner) => inner.serialize(serializer),
    }
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

  use crate::eip::eip1898::BlockHash;
  use crate::eip::eip1898::BlockId;
  use crate::eth::H256;
  use crate::eth::U64;
  use crate::rpc::BlockNumber;

  // Ethereum genesis block
  const HASH: &str = "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3";

  const TOKENS_A: &[Token] = &[
    Token::Struct {
      name: "BlockHash",
      len: 1,
    },
    Token::Str("blockHash"),
    Token::Str(HASH),
    Token::StructEnd,
  ];

  const TOKENS_B: &[Token] = &[
    Token::Struct {
      name: "BlockHash",
      len: 2,
    },
    Token::Str("blockHash"),
    Token::Str(HASH),
    Token::Str("requireCanonical"),
    Token::Bool(true),
    Token::StructEnd,
  ];

  #[test]
  fn test_block_hash_from() {
    let hash: H256 = HASH.parse().unwrap();

    assert_eq!(BlockHash::new(hash, false), hash.into());
    assert_eq!(BlockHash::new(hash, true), (hash, true).into());
    assert_eq!(BlockHash::new(hash, false), (hash, false).into());
  }

  #[test]
  fn test_block_hash_from_str() {
    let hash: H256 = HASH.parse().unwrap();
    let data: BlockHash = HASH.parse().unwrap();

    assert_eq!(data.to_hash(), hash);
  }

  #[test]
  fn test_block_hash_serde() {
    let hash: H256 = HASH.parse().unwrap();

    assert_tokens(&BlockHash::new(hash, false).readable(), TOKENS_A);
    assert_tokens(&BlockHash::new(hash, true).readable(), TOKENS_B);
  }

  #[test]
  fn test_block_id_from() {
    let hash: H256 = HASH.parse().unwrap();

    assert_eq!(BlockId::integer(0), 0.into());
    assert_eq!(BlockId::integer(0), U64::from_u64(0).into());
    assert_eq!(BlockId::earliest(), BlockNumber::Earliest.into());

    assert_eq!(BlockId::hash(hash, false), hash.into());
    assert_eq!(BlockId::hash(hash, true), (hash, true).into());
    assert_eq!(BlockId::hash(hash, false), (hash, false).into());
    assert_eq!(BlockId::hash(hash, false), BlockHash::from(hash).into());
  }

  #[test]
  fn test_block_id_from_str() {
    let hash: H256 = HASH.parse().unwrap();

    assert_eq!(BlockId::earliest(), "earliest".parse().unwrap());
    assert_eq!(BlockId::latest(), "latest".parse().unwrap());
    assert_eq!(BlockId::safe(), "safe".parse().unwrap());
    assert_eq!(BlockId::finalized(), "finalized".parse().unwrap());
    assert_eq!(BlockId::pending(), "pending".parse().unwrap());
    assert_eq!(BlockId::integer(0), "0x0".parse().unwrap());
    assert_eq!(BlockId::hash(hash, false), HASH.parse().unwrap());
  }

  #[test]
  fn test_block_id_serde() {
    let hash: H256 = HASH.parse().unwrap();

    assert_tokens(&BlockId::earliest().readable(), &[Token::Str("earliest")]);
    assert_tokens(&BlockId::latest().readable(), &[Token::Str("latest")]);
    assert_tokens(&BlockId::safe().readable(), &[Token::Str("safe")]);
    assert_tokens(&BlockId::finalized().readable(), &[Token::Str("finalized")]);
    assert_tokens(&BlockId::pending().readable(), &[Token::Str("pending")]);
    assert_tokens(&BlockId::integer(0).readable(), &[Token::Str("0x0")]);

    assert_tokens(&BlockId::hash(hash, false).readable(), TOKENS_A);
    assert_tokens(&BlockId::hash(hash, true).readable(), TOKENS_B);
  }
}
