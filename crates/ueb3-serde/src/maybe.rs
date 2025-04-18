//! A value that *might* be populated during deserialization.

use serde::Deserialize;
use serde::de::Error as _;
use serde::de::MapAccess;
use std::any;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;

/// A convenience wrapper around [`Option`] that provides easier management of
/// fields during deserialization.
pub struct Maybe<A, T> {
  access: PhantomData<A>,
  buffer: Option<T>,
}

impl<A, T> Maybe<A, T> {
  /// `Maybe` with no value.
  pub const NONE: Self = Self {
    access: PhantomData,
    buffer: None,
  };

  /// Deserializes an instance of `T` and stores the value for later retrieval.
  ///
  /// # Errors
  ///
  /// Returns `A::Error` if the value was already set.
  #[inline]
  pub fn set<'de>(&mut self, access: &mut A, name: &'static str) -> Result<(), A::Error>
  where
    T: Deserialize<'de>,
    A: MapAccess<'de>,
  {
    if self.buffer.is_some() {
      return Err(A::Error::duplicate_field(name));
    }

    self.buffer = Some(access.next_value()?);

    Ok(())
  }

  /// Converts `self` into the underlying value.
  #[inline]
  pub fn get_optional(self) -> Option<T> {
    self.buffer
  }

  /// Converts `self` into the underlying value, returning an error if the value is `None`.
  #[inline]
  pub fn get_required<'de>(self, name: &'static str) -> Result<T, A::Error>
  where
    A: MapAccess<'de>,
  {
    self
      .get_optional()
      .ok_or_else(|| A::Error::missing_field(name))
  }
}

impl<A, T> Debug for Maybe<A, T> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Maybe")
      .field("access", &any::type_name::<A>())
      .field("buffer", &any::type_name::<T>())
      .finish()
  }
}
