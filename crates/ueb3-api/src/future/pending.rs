use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::LowerHex;
use std::fmt::Result as FmtResult;
use std::fmt::UpperHex;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use ueb3_rpc::Result;
use ueb3_types::eth::H256;
use ueb3_types::eth::Slice;
use ueb3_types::rpc::TransactionReceipt;

use crate::provider::Provider;

const BYTES: usize = 32;

// -----------------------------------------------------------------------------
// Pending Transaction
// -----------------------------------------------------------------------------

/// A pending transaction future that resolves to a [`TransactionReceipt`].
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PendingTransaction<'a, T> {
  client: &'a T,
  target: H256,
  poller: Duration,
  confirm: u64,
  timeout: Option<Duration>,
}

impl<'a, T> PendingTransaction<'a, T> {
  /// Returns a reference to the JSON-RPC 2.0 client.
  #[inline]
  pub const fn client(&self) -> &'a T {
    self.client
  }

  /// Returns the transaction id.
  #[inline]
  pub const fn target(&self) -> H256 {
    self.target
  }

  /// Returns the polling interval.
  #[inline]
  pub const fn poller(&self) -> Duration {
    self.poller
  }

  /// Returns the number of confirmations required before resolving the transaction.
  #[inline]
  pub const fn confirmations(&self) -> u64 {
    self.confirm
  }

  /// Returns the maximum duration to wait for the required number of confirmations.
  #[inline]
  pub const fn timeout(&self) -> Option<Duration> {
    self.timeout
  }

  /// Sets the number of confirmations required before resolving the transaction.
  ///
  /// The default number of confirmations is `1`.
  #[inline]
  #[must_use]
  pub const fn with_confirmations(mut self, value: u64) -> Self {
    self.confirm = value;
    self
  }

  /// Sets the maximum duration to wait for the required number of confirmations.
  ///
  /// The default timeout is `None` (poll forever).
  #[inline]
  #[must_use]
  pub const fn with_timeout(mut self, value: Option<Duration>) -> Self {
    self.timeout = value;
    self
  }
}

impl<'a, T> PendingTransaction<'a, T>
where
  T: Provider,
{
  #[inline]
  pub(crate) fn new(client: &'a T, target: H256) -> Self {
    Self {
      client,
      target,
      poller: client.poller(),
      confirm: 1,
      timeout: None,
    }
  }
}

impl<T> Debug for PendingTransaction<'_, T> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    LowerHex::fmt(self, f)
  }
}

impl<T> Display for PendingTransaction<'_, T> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    LowerHex::fmt(self, f)
  }
}

impl<T> LowerHex for PendingTransaction<'_, T> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    LowerHex::fmt(&self.target, f)
  }
}

impl<T> UpperHex for PendingTransaction<'_, T> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    UpperHex::fmt(&self.target, f)
  }
}

impl<T> AsRef<Self> for PendingTransaction<'_, T> {
  #[inline]
  fn as_ref(&self) -> &Self {
    self
  }
}

impl<T> AsRef<H256> for PendingTransaction<'_, T> {
  #[inline]
  fn as_ref(&self) -> &H256 {
    self.target.as_ref()
  }
}

impl<T> AsRef<Slice> for PendingTransaction<'_, T> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    self.target.as_ref()
  }
}

impl<T> AsRef<[u8]> for PendingTransaction<'_, T> {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self.target.as_ref()
  }
}

impl<T> AsRef<[u8; BYTES]> for PendingTransaction<'_, T> {
  #[inline]
  fn as_ref(&self) -> &[u8; BYTES] {
    self.target.as_ref()
  }
}

impl<T> AsMut<Self> for PendingTransaction<'_, T> {
  #[inline]
  fn as_mut(&mut self) -> &mut Self {
    self
  }
}

impl<T> AsMut<H256> for PendingTransaction<'_, T> {
  #[inline]
  fn as_mut(&mut self) -> &mut H256 {
    self.target.as_mut()
  }
}

impl<T> AsMut<Slice> for PendingTransaction<'_, T> {
  #[inline]
  fn as_mut(&mut self) -> &mut Slice {
    self.target.as_mut()
  }
}

impl<T> AsMut<[u8]> for PendingTransaction<'_, T> {
  #[inline]
  fn as_mut(&mut self) -> &mut [u8] {
    self.target.as_mut()
  }
}

impl<T> AsMut<[u8; BYTES]> for PendingTransaction<'_, T> {
  #[inline]
  fn as_mut(&mut self) -> &mut [u8; BYTES] {
    self.target.as_mut()
  }
}

impl<T> Borrow<Slice> for PendingTransaction<'_, T> {
  #[inline]
  fn borrow(&self) -> &Slice {
    self.target.borrow()
  }
}

impl<T> BorrowMut<Slice> for PendingTransaction<'_, T> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut Slice {
    self.target.borrow_mut()
  }
}

impl<T> Borrow<[u8]> for PendingTransaction<'_, T> {
  #[inline]
  fn borrow(&self) -> &[u8] {
    self.target.borrow()
  }
}

impl<T> BorrowMut<[u8]> for PendingTransaction<'_, T> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [u8] {
    self.target.borrow_mut()
  }
}

impl<T> Borrow<[u8; BYTES]> for PendingTransaction<'_, T> {
  #[inline]
  fn borrow(&self) -> &[u8; BYTES] {
    self.target.borrow()
  }
}

impl<T> BorrowMut<[u8; BYTES]> for PendingTransaction<'_, T> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [u8; BYTES] {
    self.target.borrow_mut()
  }
}

impl<T> Future for PendingTransaction<'_, T>
where
  T: Provider,
{
  type Output = Result<TransactionReceipt>;

  #[inline]
  fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
    panic!("<PendingTransaction as Future>::poll")
  }
}
