//! Serde utility functions for quantity values.

use serde::Deserializer;
use serde::Serializer;
use ueb3_utils::traits::Quantity;

/// Deserializes `T` as a quantity value.
#[inline]
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Quantity,
{
  <T as Quantity>::deserialize(deserializer)
}

/// Serializes `T` as a quantity value.
#[inline]
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Quantity,
{
  <T as Quantity>::serialize(value, serializer)
}

pub mod opt {
  //! Serde utility functions for optional quantity values.

  use serde::Deserialize;
  use serde::Deserializer;
  use serde::Serialize;
  use serde::Serializer;
  use ueb3_utils::traits::Quantity;

  /// Deserializes `Option<T>` as a quantity value.
  #[inline]
  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
  where
    D: Deserializer<'de>,
    T: Quantity,
  {
    #[derive(Deserialize)]
    #[serde(transparent)]
    struct Value<T: Quantity> {
      #[serde(deserialize_with = "<T as Quantity>::deserialize")]
      inner: T,
    }

    let value: Option<Value<T>> = Deserialize::deserialize(deserializer)?;
    let value: Option<T> = value.map(|value| value.inner);

    Ok(value)
  }

  /// Serializes `Option<T>` as a quantity value.
  #[inline]
  pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
    T: Quantity,
  {
    struct Value<'a, T>(&'a T);

    impl<T> Serialize for Value<'_, T>
    where
      T: Quantity,
    {
      #[inline]
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: Serializer,
      {
        <T as Quantity>::serialize(self.0, serializer)
      }
    }

    if let Some(inner) = value {
      serializer.serialize_some(&Value(inner))
    } else {
      serializer.serialize_none()
    }
  }
}
