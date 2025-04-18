use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct as _;
use serde_json::value::RawValue;
use std::borrow::Borrow;
use ueb3_utils::traits::Decode;
use ueb3_utils::traits::DecodeError;

use crate::request::RequestId;
use crate::response::ResponseError;
use crate::version::VERSION;

// -----------------------------------------------------------------------------
// Response (failure)
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 error response.
#[derive(Clone, Debug)]
pub struct ResponseFailure<T> {
  pub(super) request: RequestId,
  pub(super) content: ResponseError<T>,
}

impl<T> ResponseFailure<T> {
  /// Returns the corresponding request id.
  #[inline]
  pub const fn request(&self) -> RequestId {
    self.request
  }

  /// Returns a reference to the response error object.
  #[inline]
  pub const fn content(&self) -> &ResponseError<T> {
    &self.content
  }

  /// Returns a mutable reference to the response error object.
  #[inline]
  pub const fn content_mut(&mut self) -> &mut ResponseError<T> {
    &mut self.content
  }

  /// Converts `self` into the response error object.
  #[inline]
  pub fn into_content(self) -> ResponseError<T> {
    self.content
  }

  /// Deserializes the server-defined error data as `U`.
  #[inline]
  pub fn decode<'de, U>(&'de self) -> Result<Option<U>, DecodeError>
  where
    T: Borrow<RawValue>,
    U: Decode<'de>,
  {
    self.content.decode()
  }
}

impl ResponseFailure<&RawValue> {
  /// Converts `self` into an equivalent type with boxed error data.
  #[inline]
  pub fn into_boxed(self) -> ResponseFailure<Box<RawValue>> {
    ResponseFailure {
      request: self.request,
      content: self.content.into_boxed(),
    }
  }
}

impl<T> Serialize for ResponseFailure<T>
where
  T: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state: S::SerializeStruct = serializer.serialize_struct("ResponseFailure", 3)?;
    state.serialize_field("jsonrpc", VERSION)?;
    state.serialize_field("id", &self.request)?;
    state.serialize_field("error", &self.content)?;
    state.end()
  }
}
