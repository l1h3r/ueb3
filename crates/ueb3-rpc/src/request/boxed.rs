use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct as _;
use serde_json::value::RawValue;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten as _;
use ueb3_utils::traits::Encode;
use ueb3_utils::traits::EncodeError;
use ueb3_utils::traits::OptionExt as _;

use crate::request::Request;
use crate::request::RequestHeader;
use crate::request::RequestId;

// -----------------------------------------------------------------------------
// Request Object (boxed)
// -----------------------------------------------------------------------------

/// A type-erased JSON-RPC 2.0 request object (method/id/params).
#[derive(Clone)]
pub struct BoxRequest {
  pub(super) header: RequestHeader,
  pub(super) params: Option<Box<RawValue>>,
}

impl BoxRequest {
  /// Creates a new `BoxRequest`.
  #[inline]
  pub const fn new(method: &'static str, id: RequestId, params: Option<Box<RawValue>>) -> Self {
    Self::with_header(RequestHeader::new(method, id), params)
  }

  /// Creates a new `BoxRequest` with the given `header` and `params`.
  #[inline]
  pub const fn with_header(header: RequestHeader, params: Option<Box<RawValue>>) -> Self {
    Self { header, params }
  }

  /// Returns a reference to the request header.
  #[inline]
  pub const fn header(&self) -> &RequestHeader {
    &self.header
  }

  /// Returns a mutable reference to the request header.
  #[inline]
  pub const fn header_mut(&mut self) -> &mut RequestHeader {
    &mut self.header
  }

  /// Returns a reference to the request params.
  #[inline]
  pub fn params(&self) -> &RawValue {
    self.params.as_deref().unwrap_or(RawValue::NULL)
  }
}

impl Debug for BoxRequest {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("BoxRequest")
      .field("header", &self.header)
      .field("params", &self.params.format())
      .finish()
  }
}

impl<T> TryFrom<Request<T>> for BoxRequest
where
  T: Encode,
{
  type Error = EncodeError;

  #[inline]
  fn try_from(other: Request<T>) -> Result<Self, Self::Error> {
    let params: Option<Box<RawValue>> = other.encode_params()?;
    let object: Self = Self::with_header(other.header, params);

    Ok(object)
  }
}

impl Serialize for BoxRequest {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let fields: usize = self.header.fields() + usize::from(self.params.is_some());
    let mut state: S::SerializeStruct = serializer.serialize_struct("BoxRequest", fields)?;

    self.header.serialize_fields(&mut state)?;

    if let Some(ref params) = self.params {
      state.serialize_field("params", params)?;
    } else {
      state.skip_field("params")?;
    }

    state.end()
  }
}
