use crate::error::Error;
use crate::request::BoxRequest;
use crate::request::RequestBatch;
use crate::response::BoxResponse;
use crate::response::ResponseBatch;

// -----------------------------------------------------------------------------
// RPC Transport
// -----------------------------------------------------------------------------

/// A low-level interface for JSON-RPC 2.0 request/response management.
pub trait RpcTransport {
  /// Execute a single JSON-RPC 2.0 request.
  fn call(&self, value: BoxRequest) -> Result<BoxResponse, Error>;

  /// Execute a batch of JSON-RPC 2.0 requests.
  ///
  /// The default implementation will repeatedly dispatch requests with
  /// [`call`][Self::call] and return an error if **any** of the requests fail.
  #[inline]
  fn call_many(&self, value: Vec<BoxRequest>) -> Result<Vec<BoxResponse>, Error> {
    let mut output: Vec<BoxResponse> = Vec::with_capacity(value.len());

    for value in value {
      output.push(self.call(value)?);
    }

    Ok(output)
  }

  /// Execute a potential batch of JSON-RPC 2.0 requests.
  ///
  /// Note: Implementations should avoid overriding this and instead override
  /// the default implementation of [`call_many`][Self::call_many].
  #[inline]
  fn call_batch(&self, value: RequestBatch) -> Result<ResponseBatch, Error> {
    match value {
      RequestBatch::Batch(inner) => self.call_many(inner).map(ResponseBatch::Batch),
      RequestBatch::Value(inner) => self.call(inner).map(ResponseBatch::Value),
    }
  }

  /// Converts `self` into a boxed `RpcTransport`.
  #[inline]
  fn boxed(self) -> Box<dyn RpcTransport>
  where
    Self: Sized + 'static,
  {
    Box::new(self)
  }
}

impl RpcTransport for Box<dyn RpcTransport> {
  #[inline]
  fn call(&self, value: BoxRequest) -> Result<BoxResponse, Error> {
    (**self).call(value)
  }

  #[inline]
  fn boxed(self) -> Box<dyn RpcTransport> {
    self
  }
}
