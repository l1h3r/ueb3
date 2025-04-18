use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use serde::de::IntoDeserializer as _;
use serde::de::Visitor;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;
use ueb3_utils::base16::FromHex;
use ueb3_utils::base16::FromHexError;

use crate::eth::U64;

// -----------------------------------------------------------------------------
// Ethereum Block Number/Tag
// -----------------------------------------------------------------------------

/// A block number or tag.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum BlockNumber {
  /// The earliest/genesis block.
  Earliest,
  /// The latest proposed block.
  #[default]
  Latest,
  /// The latest safe head block.
  Safe,
  /// The latest finalized block.
  Finalized,
  /// The pending state/transactions.
  Pending,
  /// Integer block number.
  Integer(u64),
}

impl BlockNumber {
  /// Returns `true` if `self` is a tag referencing the earliest/genesis block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockNumber::Earliest.is_earliest(), true);
  /// assert_eq!(BlockNumber::Integer(0).is_earliest(), false);
  /// ```
  #[inline]
  pub const fn is_earliest(&self) -> bool {
    matches!(self, Self::Earliest)
  }

  /// Returns `true` if `self` is a tag referencing the latest proposed block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockNumber::Latest.is_latest(), true);
  /// assert_eq!(BlockNumber::Integer(0).is_latest(), false);
  /// ```
  #[inline]
  pub const fn is_latest(&self) -> bool {
    matches!(self, Self::Latest)
  }

  /// Returns `true` if `self` is a tag referencing the latest safe head block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockNumber::Safe.is_safe(), true);
  /// assert_eq!(BlockNumber::Integer(0).is_safe(), false);
  /// ```
  #[inline]
  pub const fn is_safe(&self) -> bool {
    matches!(self, Self::Safe)
  }

  /// Returns `true` if `self` is a tag referencing the latest finalized block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockNumber::Finalized.is_finalized(), true);
  /// assert_eq!(BlockNumber::Integer(0).is_finalized(), false);
  /// ```
  #[inline]
  pub const fn is_finalized(&self) -> bool {
    matches!(self, Self::Finalized)
  }

  /// Returns `true` if `self` is a tag referencing the pending state.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockNumber::Pending.is_pending(), true);
  /// assert_eq!(BlockNumber::Integer(0).is_pending(), false);
  /// ```
  #[inline]
  pub const fn is_pending(&self) -> bool {
    matches!(self, Self::Pending)
  }

  /// Returns `true` if `self` is an integer referencing a specific block.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockNumber::Integer(0).is_integer(), true);
  /// assert_eq!(BlockNumber::Earliest.is_integer(), false);
  /// ```
  #[inline]
  pub const fn is_integer(&self) -> bool {
    matches!(self, Self::Integer(_))
  }

  /// Converts `self` to a 64-bit integer.
  ///
  /// This may return `None` if the block number is a tag variant.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::BlockNumber;
  ///
  /// assert_eq!(BlockNumber::Integer(0).to_integer(), Some(0));
  /// assert_eq!(BlockNumber::Earliest.to_integer(), None);
  /// assert_eq!(BlockNumber::Latest.to_integer(), None);
  /// assert_eq!(BlockNumber::Safe.to_integer(), None);
  /// assert_eq!(BlockNumber::Finalized.to_integer(), None);
  /// assert_eq!(BlockNumber::Pending.to_integer(), None);
  /// ```
  #[inline]
  pub const fn to_integer(&self) -> Option<u64> {
    match self {
      Self::Earliest => None,
      Self::Latest => None,
      Self::Safe => None,
      Self::Finalized => None,
      Self::Pending => None,
      Self::Integer(inner) => Some(*inner),
    }
  }

  #[inline]
  const fn as_str(&self) -> &'static str {
    match self {
      Self::Earliest => "earliest",
      Self::Latest => "latest",
      Self::Safe => "safe",
      Self::Finalized => "finalized",
      Self::Pending => "pending",
      Self::Integer(_) => "",
    }
  }
}

impl Debug for BlockNumber {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Earliest => f.write_str("Earliest"),
      Self::Latest => f.write_str("Latest"),
      Self::Safe => f.write_str("Safe"),
      Self::Finalized => f.write_str("Finalized"),
      Self::Pending => f.write_str("Pending"),
      Self::Integer(inner) => Debug::fmt(inner, f),
    }
  }
}

impl Display for BlockNumber {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if let Self::Integer(inner) = self {
      return Display::fmt(inner, f);
    }

    f.write_str(self.as_str())
  }
}

impl From<u64> for BlockNumber {
  #[inline]
  fn from(other: u64) -> Self {
    Self::Integer(other)
  }
}

impl From<U64> for BlockNumber {
  #[inline]
  fn from(other: U64) -> Self {
    Self::Integer(other.into_u64())
  }
}

impl FromStr for BlockNumber {
  type Err = FromHexError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "earliest" => Ok(Self::Earliest),
      "latest" => Ok(Self::Latest),
      "safe" => Ok(Self::Safe),
      "finalized" => Ok(Self::Finalized),
      "pending" => Ok(Self::Pending),
      _ => FromHex::from_str(input),
    }
  }
}

impl FromHex for BlockNumber {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    U64::from_slice(input).map(Into::into)
  }
}

impl<'de> Deserialize<'de> for BlockNumber {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl Visitor<'_> for __Visitor {
      type Value = BlockNumber;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a hex-encoded block number or tag")
      }

      fn visit_bytes<E>(self, data: &[u8]) -> Result<Self::Value, E>
      where
        E: Error,
      {
        match data {
          b"earliest" => Ok(BlockNumber::Earliest),
          b"latest" => Ok(BlockNumber::Latest),
          b"safe" => Ok(BlockNumber::Safe),
          b"finalized" => Ok(BlockNumber::Finalized),
          b"pending" => Ok(BlockNumber::Pending),
          _ => U64::deserialize(data.into_deserializer()).map(Into::into),
        }
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: Error,
      {
        value.parse().map_err(E::custom)
      }
    }

    if deserializer.is_human_readable() {
      deserializer.deserialize_str(__Visitor)
    } else {
      deserializer.deserialize_bytes(__Visitor)
    }
  }
}

impl Serialize for BlockNumber {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    if let Self::Integer(inner) = self {
      return U64::from_u64(*inner).serialize(serializer);
    }

    if serializer.is_human_readable() {
      serializer.serialize_str(self.as_str())
    } else {
      serializer.serialize_bytes(self.as_str().as_bytes())
    }
  }
}

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eth::U64;
  use crate::rpc::BlockNumber;

  #[test]
  fn test_from() {
    assert_eq!(BlockNumber::Integer(0), 0.into());
    assert_eq!(BlockNumber::Integer(0), U64::from_u64(0).into());
  }

  #[test]
  fn test_from_str() {
    assert_eq!(BlockNumber::Earliest, "earliest".parse().unwrap());
    assert_eq!(BlockNumber::Latest, "latest".parse().unwrap());
    assert_eq!(BlockNumber::Safe, "safe".parse().unwrap());
    assert_eq!(BlockNumber::Finalized, "finalized".parse().unwrap());
    assert_eq!(BlockNumber::Pending, "pending".parse().unwrap());
    assert_eq!(BlockNumber::Integer(0), "0x0".parse().unwrap());
  }

  #[rustfmt::skip]
  #[test]
  fn test_serde() {
    assert_tokens(&BlockNumber::Earliest.readable(), &[Token::Str("earliest")]);
    assert_tokens(&BlockNumber::Latest.readable(), &[Token::Str("latest")]);
    assert_tokens(&BlockNumber::Safe.readable(), &[Token::Str("safe")]);
    assert_tokens(&BlockNumber::Finalized.readable(), &[Token::Str("finalized")]);
    assert_tokens(&BlockNumber::Pending.readable(), &[Token::Str("pending")]);
    assert_tokens(&BlockNumber::Integer(0).readable(), &[Token::Str("0x0")]);

    assert_tokens(&BlockNumber::Earliest.compact(), &[Token::Bytes(b"earliest")]);
    assert_tokens(&BlockNumber::Latest.compact(), &[Token::Bytes(b"latest")]);
    assert_tokens(&BlockNumber::Safe.compact(), &[Token::Bytes(b"safe")]);
    assert_tokens(&BlockNumber::Finalized.compact(), &[Token::Bytes(b"finalized")]);
    assert_tokens(&BlockNumber::Pending.compact(), &[Token::Bytes(b"pending")]);
    assert_tokens(&BlockNumber::Integer(0).compact(), &[Token::Bytes(&[0, 0, 0, 0, 0, 0, 0, 0])]);
  }
}
