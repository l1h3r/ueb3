//! Serde extension trait for struct flattening.

use serde::Serializer;
use serde::ser::SerializeStruct;

// -----------------------------------------------------------------------------
// Serialize Flatten
// -----------------------------------------------------------------------------

/// A convenience wrapper around [`serde`] serialization that provides
/// the ability to easily flatten nested structs.
pub trait SerializeFlatten {
  /// The name of the `struct` to serialize.
  const NAME: &'static str;

  /// The number of fields that will be serialized.
  fn fields(&self) -> usize;

  /// Serializes all fields into the given `state`.
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct;

  /// Serializes `self` as a `struct` with all fields.
  #[inline]
  fn serialize_struct<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state: S::SerializeStruct = serializer.serialize_struct(Self::NAME, self.fields())?;

    self.serialize_fields(&mut state)?;

    state.end()
  }
}
