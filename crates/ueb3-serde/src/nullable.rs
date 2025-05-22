//! Serde utility functions for possibly `null` values.

pub mod sequence {
  //! Deserializes/Serializes a sequence of `T` that might be received as `null`.

  use serde::Deserialize;
  use serde::Deserializer;
  use serde::Serialize;
  use serde::Serializer;

  /// Deserializes a sequence of `T` that might be received as `null`.
  #[inline]
  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
  where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
  {
    Deserialize::deserialize(deserializer).map(Option::unwrap_or_default)
  }

  /// Serializes a sequence of `T`.
  #[inline]
  pub fn serialize<S, T>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: Serialize,
    S: Serializer,
  {
    Serialize::serialize(&Some(value), serializer)
  }
}

pub mod default {
  //! Deserializes/Serializes an instance of `T` that might be received as `null`.

  use serde::Deserialize;
  use serde::Deserializer;
  use serde::Serialize;
  use serde::Serializer;

  /// Deserializes an instance of `T` that might be received as `null`.
  #[inline]
  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
  where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
  {
    Deserialize::deserialize(deserializer).map(Option::unwrap_or_default)
  }

  /// Serializes an instance of `T`.
  #[inline]
  pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: Serialize,
    S: Serializer,
  {
    Serialize::serialize(&Some(value), serializer)
  }
}
