use std::any;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;
use ueb3_rpc::Result;
use ueb3_types::rpc::TransactionLog;
use ueb3_utils::feature;
use ueb3_utils::traits::DecodeOwned;

use crate::provider::Provider;
use crate::stream::FilterConfig;
use crate::stream::FilterId;

feature! {
  #[feature = "streams"]
  use async_stream::stream;
  use futures_core::stream::Stream;
  use futures_timer::Delay;
  use std::time::Duration;
  use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Filter Stream
// -----------------------------------------------------------------------------

/// A streaming interface that polls filter changes from a remote node.
///
/// This `struct` is created by one of the following methods:
///
/// * [`watch_logs`][Provider::watch_logs]
/// * [`watch_blocks`][Provider::watch_blocks]
/// * [`watch_transactions`][Provider::watch_transactions]
///
/// You should call [`uninstall`][Self::uninstall] to free node resources when
/// you no longer need this object.
pub struct FilterStream<'a, T, U> {
  client: &'a T,
  filter: FilterId,
  config: FilterConfig,
  marker: PhantomData<U>,
}

impl<'a, T, U> FilterStream<'a, T, U> {
  /// Returns a reference to the JSON-RPC 2.0 client.
  #[inline]
  pub const fn client(&self) -> &'a T {
    self.client
  }

  /// Returns the filter id.
  #[inline]
  pub const fn filter(&self) -> FilterId {
    self.filter
  }

  /// Returns a reference to the filter config.
  #[inline]
  pub const fn config(&self) -> &FilterConfig {
    &self.config
  }

  /// Sets the filter config.
  #[inline]
  #[must_use]
  pub const fn with_config(mut self, value: FilterConfig) -> Self {
    self.config = value;
    self
  }
}

impl<'a, T, U> FilterStream<'a, T, U>
where
  T: Provider,
{
  #[inline]
  pub(crate) fn new(client: &'a T, filter: FilterId) -> Self {
    Self {
      client,
      filter,
      config: FilterConfig::new().with_interval(client.poller()),
      marker: PhantomData,
    }
  }

  /// Uninstalls the filter.
  #[doc(alias = "eth_uninstallFilter")]
  #[inline]
  pub fn uninstall(self) -> Result<bool> {
    self.client.uninstall_filter(self.filter)
  }
}

impl<T, U> FilterStream<'_, T, U>
where
  T: Provider,
  U: DecodeOwned,
{
  /// Returns an array of logs which occurred since last poll.
  #[doc(alias = "eth_getFilterChanges")]
  #[inline]
  pub fn changes(&self) -> Result<Vec<U>> {
    self.client.filter_changes(self.filter)
  }

  /// Converts `self` to a [`Stream`][Stream] of `U`.
  ///
  /// The resulting stream will periodically poll the underlying provider for
  /// changes at the configured interval.
  ///
  /// Note: Due to semantics and lack of `AsyncDrop`, you should consider
  /// storing the filter [`id`][Self::filter] separately before calling this
  /// function - afterwards you can properly remove the filter with
  /// [`Provider::uninstall_filter`].
  #[cfg(feature = "streams")]
  #[inline]
  pub fn into_stream(self) -> impl Stream<Item = Result<U>> {
    #![allow(tail_expr_drop_order)]
    stream! {
      let interval: Duration = self.config.interval();
      let max_poll: Option<usize> = self.config.max_poll();

      let mut delay: Delay = Delay::new(interval);
      let mut timer: Instant;
      let mut index: usize = 0;

      loop {
        timer = Instant::now();
        index += 1;

        match self.changes() {
          Ok(items) => {
            for item in items {
              yield Ok(item);
            }
          }
          Err(error) => {
            yield Err(error);
          }
        }

        if matches!(max_poll, Some(max_poll) if max_poll == index) {
          break;
        }

        if let Some(value) = interval.checked_sub(timer.elapsed()) {
          delay.reset(value);
          (&mut delay).await;
        }
      }
    }
  }
}

impl<T> FilterStream<'_, T, TransactionLog>
where
  T: Provider,
{
  /// Returns an array of all filter logs.
  #[doc(alias = "eth_getFilterLogs")]
  #[inline]
  pub fn logs(&self) -> Result<Vec<TransactionLog>> {
    self.client.filter_logs(self.filter)
  }
}

impl<T, U> Debug for FilterStream<'_, T, U> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("FilterStream")
      .field("filter", &self.filter)
      .field("config", &self.config)
      .field("marker", &any::type_name::<U>())
      .finish_non_exhaustive()
  }
}
