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

    if self.result.is_some() {
      state.serialize_field("result", &self.result)?;
    } else {
      state.skip_field("result")?;
    }

    if self.error.is_some() {
      state.serialize_field("error", &self.error)?;
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::TracePayload;

  type Data = (u32, u32);

  #[test]
  fn test_serde() {
    let a: TracePayload<(), ()> = TracePayload::default();

    let b: TracePayload<Data, Data> = TracePayload {
      subtraces: 2,
      trace_address: vec![0, 3, 5],
      action: (12, 34),
      result: Some((56, 78)),
      error: None,
    };

    let c: TracePayload<(), ()> = TracePayload {
      error: Some("test".into()),
      ..TracePayload::default()
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TracePayload",
        len: 3,
      },
      Token::Str("subtraces"),
      Token::U64(0),
      Token::Str("traceAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("action"),
      Token::Unit,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TracePayload",
        len: 4,
      },
      Token::Str("subtraces"),
      Token::U64(2),
      Token::Str("traceAddress"),
      Token::Seq { len: Some(3) },
      Token::U64(0),
      Token::U64(3),
      Token::U64(5),
      Token::SeqEnd,
      Token::Str("action"),
      Token::Tuple { len: 2 },
      Token::U32(12),
      Token::U32(34),
      Token::TupleEnd,
      Token::Str("result"),
      Token::Some,
      Token::Tuple { len: 2 },
      Token::U32(56),
      Token::U32(78),
      Token::TupleEnd,
      Token::StructEnd,
    ];

    let tokens_c: &[Token] = &[
      Token::Struct {
        name: "TracePayload",
        len: 4,
      },
      Token::Str("subtraces"),
      Token::U64(0),
      Token::Str("traceAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("action"),
      Token::Unit,
      Token::Str("error"),
      Token::Some,
      Token::Str("test"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
    assert_tokens(&c.readable(), tokens_c);
  }
}
