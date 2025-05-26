use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten;
use ueb3_utils::traits::OptionExt as _;

// -----------------------------------------------------------------------------
// Parity Trace Output
// -----------------------------------------------------------------------------

/// A typed trace object.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TracePayload<A, R> {
  /// The total number of relevant sub-traces.
  pub subtraces: u64,
  /// The exact location in the call trace.
  pub trace_address: Vec<u64>,
  /// Type-specific trace information.
  pub action: A,
  /// Type-specific trace output.
  pub result: Option<R>,
  /// The error associated with the transaction.
  pub error: Option<String>,
}

impl<A, R> Debug for TracePayload<A, R>
where
  A: Debug,
  R: Debug,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TracePayload")
      .field("subtraces", &self.subtraces)
      .field("trace_address", &format_args!("{:?}", self.trace_address))
      .field("action", &self.action)
      .field("result", &self.result.format())
      .field("error", &self.error.format())
      .finish()
  }
}

impl<A, R> SerializeFlatten for TracePayload<A, R>
where
  A: Serialize,
  R: Serialize,
{
  const NAME: &'static str = "TracePayload";

  #[inline]
  fn fields(&self) -> usize {
    3 + usize::from(self.result.is_some()) + usize::from(self.error.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("subtraces", &self.subtraces)?;
    state.serialize_field("traceAddress", &self.trace_address)?;
    state.serialize_field("action", &self.action)?;

    if let Some(ref value) = self.result {
      state.serialize_field("result", value)?;
    } else {
      state.skip_field("result")?;
    }

    if let Some(ref value) = self.error {
      state.serialize_field("error", value)?;
    } else {
      state.skip_field("error")?;
    }

    Ok(())
  }
}

impl<A, R> Serialize for TracePayload<A, R>
where
  A: Serialize,
  R: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.serialize_struct(serializer)
  }
}
