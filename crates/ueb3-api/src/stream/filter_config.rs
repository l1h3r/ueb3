use std::time::Duration;

// -----------------------------------------------------------------------------
// Filter COnfiguration
// -----------------------------------------------------------------------------

/// The configuration for a [`FilterStream`].
///
/// [`FilterStream`]: crate::stream::FilterStream
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FilterConfig {
  max_poll: Option<usize>,
  interval: Duration,
}

impl FilterConfig {
  /// Creates a new `FilterConfig`.
  #[inline]
  pub const fn new() -> Self {
    Self {
      max_poll: None,
      interval: Duration::from_secs(1),
    }
  }

  /// Returns the maximum number of polls.
  #[inline]
  pub const fn max_poll(&self) -> Option<usize> {
    self.max_poll
  }

  /// Returns the polling interval.
  #[inline]
  pub const fn interval(&self) -> Duration {
    self.interval
  }

  /// Sets the maximum number of polls.
  #[inline]
  #[must_use]
  pub const fn with_max_poll(mut self, value: Option<usize>) -> Self {
    self.max_poll = value;
    self
  }

  /// Sets the polling interval.
  #[inline]
  #[must_use]
  pub const fn with_interval(mut self, value: Duration) -> Self {
    self.interval = value;
    self
  }
}

impl Default for FilterConfig {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
