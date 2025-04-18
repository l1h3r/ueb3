// -----------------------------------------------------------------------------
// Size Properties
// -----------------------------------------------------------------------------

/// Provides associated constants for useful size-related type properties.
pub trait SizeOf: Sized {
  /// `true` if this type is zero-sized.
  const IS_ZST: bool = size_of::<Self>() == 0;
}

impl<T> SizeOf for T {}
