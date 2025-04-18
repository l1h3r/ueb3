use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for &T {}
impl<T> private::Sealed for Option<T> {}

// -----------------------------------------------------------------------------
// Option Formatter
// -----------------------------------------------------------------------------

/// A wrapper around [`Option`] that prints unwrapped values.
///
/// This `struct` is created by the [`OptionExt::format`] function.
#[repr(transparent)]
pub struct OptionFmt<'a, T>(Option<&'a T>);

impl<T> Debug for OptionFmt<'_, T>
where
  T: Debug,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let Some(value) = self.0 else {
      return f.write_str("None");
    };

    Debug::fmt(value, f)
  }
}

impl<T> Display for OptionFmt<'_, T>
where
  T: Display,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let Some(value) = self.0 else {
      return f.write_str("None");
    };

    Display::fmt(value, f)
  }
}

// -----------------------------------------------------------------------------
// Option Extension
// -----------------------------------------------------------------------------

/// Extension trait for [`Option`] formatting.
///
/// # Examples
///
/// ```
/// use ueb3_utils::traits::OptionExt;
///
/// assert_eq!(format!("{}", Some(123).format()), "123");
/// assert_eq!(format!("{}", None::<u32>.format()), "None");
/// ```
pub trait OptionExt<T>: private::Sealed {
  /// Returns a [`Debug`]/[`Display`]-compatible type that prints unwrapped values.
  ///
  /// * [`Some`] values are unwrapped and the inner value is printed as-is.
  /// * [`None`] values are printed as `None`.
  fn format(&self) -> OptionFmt<'_, T>;
}

impl<T> OptionExt<T> for &T
where
  T: OptionExt<T>,
{
  #[inline]
  fn format(&self) -> OptionFmt<'_, T> {
    (**self).format()
  }
}

impl<T> OptionExt<T> for Option<T> {
  #[inline]
  fn format(&self) -> OptionFmt<'_, T> {
    OptionFmt(self.as_ref())
  }
}
