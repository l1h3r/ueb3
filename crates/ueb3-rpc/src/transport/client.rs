use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use ueb3_utils::traits::DecodeOwned;
use ueb3_utils::traits::Encode;

use crate::error::Error;
use crate::request::BoxRequest;
use crate::request::Request;
use crate::request::RequestHeader;
use crate::request::RequestId;
use crate::response::BoxResponse;
use crate::transport::RpcTransport;

// -----------------------------------------------------------------------------
// RPC Client
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 client interface.
///
/// This type manages request/response construction and dispatching
/// requests to the underlying [`RpcTransport`].
pub struct RpcClient {
  inner: Box<dyn RpcTransport>,
  nonce: AtomicU64,
}

impl RpcClient {
  /// Creates a new `RpcClient`.
  #[inline]
  pub fn new<T>(transport: T) -> Self
  where
    T: RpcTransport + 'static,
  {
    Self::with_nonce(transport, 0)
  }

  /// Creates a new `RpcClient` with the given `nonce`.
  #[inline]
  pub fn with_nonce<T>(transport: T, nonce: u64) -> Self
  where
    T: RpcTransport + 'static,
  {
    Self {
      inner: transport.boxed(),
      nonce: AtomicU64::new(nonce),
    }
  }

  /// Returns a reference to the underlying `transport`.
  #[inline]
  pub const fn transport(&self) -> &dyn RpcTransport {
    &*self.inner
  }

  /// Returns a mutable reference to the underlying `transport`.
  #[inline]
  pub const fn transport_mut(&mut self) -> &mut dyn RpcTransport {
    &mut *self.inner
  }

  /// Converts `self` into the underlying `transport`.
  #[inline]
  pub fn into_transport(self) -> Box<dyn RpcTransport> {
    self.inner
  }

  /// Execute a single JSON-RPC 2.0 request.
  #[inline]
  pub fn rpc<P, R>(&self, method: &'static str, params: P) -> Result<R, Error>
  where
    P: Encode,
    R: DecodeOwned,
  {
    let object: BoxRequest = self.build(method, params)?;
    let result: BoxResponse = self.inner.call(object)?;

    match result {
      BoxResponse::Success(inner) => inner.decode().map_err(Into::into),
      BoxResponse::Failure(inner) => Err(inner.into_content().into()),
    }
  }

  #[inline]
  fn build<P>(&self, method: &'static str, params: P) -> Result<BoxRequest, Error>
  where
    P: Encode,
  {
    let header: RequestHeader = RequestHeader::new(method, self.nonce());
    let params: Request<P> = Request::with_header(header, params);
    let object: BoxRequest = params.try_into()?;

    Ok(object)
  }

  #[inline]
  fn nonce(&self) -> RequestId {
    RequestId::new(self.nonce.fetch_add(1, Ordering::SeqCst))
  }
}

impl Debug for RpcClient {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("RpcClient")
      .field("nonce", &self.nonce)
      .finish_non_exhaustive()
  }
}
