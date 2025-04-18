use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use serde::de::MapAccess;
use serde::de::Unexpected;
use serde::de::Visitor;
use serde::de::value::MapAccessDeserializer;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;

use crate::rpc::SyncStatus;

// -----------------------------------------------------------------------------
// Ethereum Node Sync Result
// -----------------------------------------------------------------------------

/// A snapshot of the current sync state of a node.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SyncResult<T> {
  /// Node syncing is disabled.
  #[default]
  Disabled,
  /// Node syncing is enabled.
  Enabled(SyncStatus<T>),
}

impl<T> SyncResult<T> {
  /// Returns `true` if `self` represents a disabled sync state.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::SyncResult;
  /// use ueb3_types::rpc::SyncStatus;
  ///
  /// let a: SyncResult<()> = SyncResult::Disabled;
  /// let b: SyncResult<()> = SyncResult::Enabled(SyncStatus::default());
  ///
  /// assert_eq!(a.is_disabled(), true);
  /// assert_eq!(b.is_disabled(), false);
  /// ```
  #[inline]
  pub const fn is_disabled(&self) -> bool {
    matches!(self, Self::Disabled)
  }

  /// Returns `true` if `self` represents an enabled sync state.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::SyncResult;
  /// use ueb3_types::rpc::SyncStatus;
  ///
  /// let a: SyncResult<()> = SyncResult::Enabled(SyncStatus::default());
  /// let b: SyncResult<()> = SyncResult::Disabled;
  ///
  /// assert_eq!(a.is_enabled(), true);
  /// assert_eq!(b.is_enabled(), false);
  /// ```
  #[inline]
  pub const fn is_enabled(&self) -> bool {
    matches!(self, Self::Enabled(_))
  }
}

impl<'de, T> Deserialize<'de> for SyncResult<T>
where
  T: Deserialize<'de>,
{
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for __Visitor<T>
    where
      T: Deserialize<'de>,
    {
      type Value = SyncResult<T>;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sync status object or `false`")
      }

      fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
      where
        E: Error,
      {
        if value {
          Err(E::invalid_value(Unexpected::Bool(value), &self))
        } else {
          Ok(SyncResult::Disabled)
        }
      }

      fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let access: MapAccessDeserializer<A> = MapAccessDeserializer::new(access);
        let status: SyncStatus<T> = Deserialize::deserialize(access)?;

        Ok(SyncResult::Enabled(status))
      }
    }

    deserializer.deserialize_any(__Visitor(PhantomData))
  }
}

impl<T> Serialize for SyncResult<T>
where
  T: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Disabled => serializer.serialize_bool(false),
      Self::Enabled(inner) => inner.serialize(serializer),
    }
  }
}

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::SyncResult;
  use crate::rpc::SyncStatus;

  #[test]
  fn test_serde() {
    let result: SyncResult<()> = SyncResult::Disabled;

    assert_tokens(&result, &[Token::Bool(false)]);

    let result: SyncResult<()> = SyncResult::Enabled(SyncStatus::default());

    assert_tokens(
      &result.readable(),
      &[
        Token::Map { len: None },
        Token::Str("startingBlock"),
        Token::Str("0x0"),
        Token::Str("currentBlock"),
        Token::Str("0x0"),
        Token::Str("highestBlock"),
        Token::Str("0x0"),
        Token::MapEnd,
      ],
    );
  }
}
