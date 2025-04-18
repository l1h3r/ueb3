use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use ueb3_serde::flatten::SerializeFlatten;

use crate::request::RequestId;
use crate::version::VERSION;

// -----------------------------------------------------------------------------
// Request Header
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 request header (method/id).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct RequestHeader {
  method: &'static str,
  id: RequestId,
}

impl RequestHeader {
  /// Creates a new `RequestHeader`.
  #[inline]
  pub const fn new(method: &'static str, id: RequestId) -> Self {
    Self { method, id }
  }

  /// Returns the request method.
  #[inline]
  pub const fn method(&self) -> &'static str {
    self.method
  }

  /// Returns the request id.
  #[inline]
  pub const fn id(&self) -> RequestId {
    self.id
  }
}

impl SerializeFlatten for RequestHeader {
  const NAME: &'static str = "RequestHeader";

  #[inline]
  fn fields(&self) -> usize {
    3
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("jsonrpc", VERSION)?;
    state.serialize_field("method", self.method)?;
    state.serialize_field("id", &self.id)?;

    Ok(())
  }
}

impl Serialize for RequestHeader {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.serialize_struct(serializer)
  }
}
