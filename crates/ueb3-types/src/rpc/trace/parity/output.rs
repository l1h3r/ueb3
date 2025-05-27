use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten;

use crate::rpc::trace::TraceActionCall;
use crate::rpc::trace::TraceActionCreate;
use crate::rpc::trace::TraceActionReward;
use crate::rpc::trace::TraceActionSuicide;
use crate::rpc::trace::TraceCategory;
use crate::rpc::trace::TracePayload;
use crate::rpc::trace::TraceResultCall;
use crate::rpc::trace::TraceResultCreate;

// -----------------------------------------------------------------------------
// Parity Trace Output
// -----------------------------------------------------------------------------

/// A dynamic wrapper around a typed trace object.
#[derive(Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum TraceOutput {
  /// `call` trace type.
  Call(TracePayload<TraceActionCall, TraceResultCall>),
  /// `create` trace type.
  Create(TracePayload<TraceActionCreate, TraceResultCreate>),
  /// `reward` trace type.
  Reward(TracePayload<TraceActionReward, ()>),
  /// `suicide` trace type.
  Suicide(TracePayload<TraceActionSuicide, ()>),
}

impl TraceOutput {
  /// Returns the trace type.
  #[inline]
  pub const fn kind(&self) -> TraceCategory {
    match self {
      Self::Call(_) => TraceCategory::Call,
      Self::Create(_) => TraceCategory::Create,
      Self::Reward(_) => TraceCategory::Reward,
      Self::Suicide(_) => TraceCategory::Suicide,
    }
  }

  /// Returns the total number of relevant sub-traces.
  #[inline]
  pub const fn subtraces(&self) -> u64 {
    match self {
      Self::Call(inner) => inner.subtraces,
      Self::Create(inner) => inner.subtraces,
      Self::Reward(inner) => inner.subtraces,
      Self::Suicide(inner) => inner.subtraces,
    }
  }

  /// Returns the exact location in the call trace.
  #[inline]
  pub const fn trace_address(&self) -> &[u64] {
    match self {
      Self::Call(inner) => inner.trace_address.as_slice(),
      Self::Create(inner) => inner.trace_address.as_slice(),
      Self::Reward(inner) => inner.trace_address.as_slice(),
      Self::Suicide(inner) => inner.trace_address.as_slice(),
    }
  }

  /// Returns the trace action of a `call` trace.
  #[inline]
  pub const fn action_call(&self) -> Option<&TraceActionCall> {
    let Self::Call(inner) = self else {
      return None;
    };

    Some(&inner.action)
  }

  /// Returns the trace action of a `create` trace.
  #[inline]
  pub const fn action_create(&self) -> Option<&TraceActionCreate> {
    let Self::Create(inner) = self else {
      return None;
    };

    Some(&inner.action)
  }

  /// Returns the trace action of a `reward` trace.
  #[inline]
  pub const fn action_reward(&self) -> Option<&TraceActionReward> {
    let Self::Reward(inner) = self else {
      return None;
    };

    Some(&inner.action)
  }

  /// Returns the trace action of a `suicide` trace.
  #[inline]
  pub const fn action_suicide(&self) -> Option<&TraceActionSuicide> {
    let Self::Suicide(inner) = self else {
      return None;
    };

    Some(&inner.action)
  }

  /// Returns the trace result of a `call` trace.
  #[inline]
  pub const fn result_call(&self) -> Option<&TraceResultCall> {
    let Self::Call(inner) = self else {
      return None;
    };

    inner.result.as_ref()
  }

  /// Returns the trace result of a `create` trace.
  #[inline]
  pub const fn result_create(&self) -> Option<&TraceResultCreate> {
    let Self::Create(inner) = self else {
      return None;
    };

    inner.result.as_ref()
  }

  /// Returns the error associated with the transaction.
  #[inline]
  pub fn error(&self) -> Option<&str> {
    match self {
      Self::Call(inner) => inner.error.as_deref(),
      Self::Create(inner) => inner.error.as_deref(),
      Self::Reward(inner) => inner.error.as_deref(),
      Self::Suicide(inner) => inner.error.as_deref(),
    }
  }
}

impl Debug for TraceOutput {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Call(inner) => Debug::fmt(inner, f),
      Self::Create(inner) => Debug::fmt(inner, f),
      Self::Reward(inner) => Debug::fmt(inner, f),
      Self::Suicide(inner) => Debug::fmt(inner, f),
    }
  }
}

impl SerializeFlatten for TraceOutput {
  const NAME: &'static str = "TraceOutput";

  #[inline]
  fn fields(&self) -> usize {
    match self {
      Self::Call(inner) => inner.fields() + 1,
      Self::Create(inner) => inner.fields() + 1,
      Self::Reward(inner) => inner.fields() + 1,
      Self::Suicide(inner) => inner.fields() + 1,
    }
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("type", &self.kind())?;

    match self {
      Self::Call(inner) => inner.serialize_fields(state),
      Self::Create(inner) => inner.serialize_fields(state),
      Self::Reward(inner) => inner.serialize_fields(state),
      Self::Suicide(inner) => inner.serialize_fields(state),
    }
  }
}

impl Serialize for TraceOutput {
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

  use crate::rpc::trace::TraceOutput;

  #[test]
  fn test_serde() {
    let a: TraceOutput = TraceOutput::Call(Default::default());
    let b: TraceOutput = TraceOutput::Create(Default::default());
    let c: TraceOutput = TraceOutput::Reward(Default::default());
    let d: TraceOutput = TraceOutput::Suicide(Default::default());

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceOutput",
        len: 4,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TraceCategory",
        variant: "call",
      },
      Token::Str("subtraces"),
      Token::U64(0),
      Token::Str("traceAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("action"),
      Token::Struct {
        name: "TraceActionCall",
        len: 6,
      },
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("callType"),
      Token::UnitVariant {
        name: "CallType",
        variant: "call",
      },
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("to"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceOutput",
        len: 4,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TraceCategory",
        variant: "create",
      },
      Token::Str("subtraces"),
      Token::U64(0),
      Token::Str("traceAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("action"),
      Token::Struct {
        name: "TraceActionCreate",
        len: 4,
      },
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("init"),
      Token::Str("0x"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::StructEnd,
    ];

    let tokens_c: &[Token] = &[
      Token::Struct {
        name: "TraceOutput",
        len: 4,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TraceCategory",
        variant: "reward",
      },
      Token::Str("subtraces"),
      Token::U64(0),
      Token::Str("traceAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("action"),
      Token::Struct {
        name: "TraceActionReward",
        len: 3,
      },
      Token::Str("author"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("rewardType"),
      Token::UnitVariant {
        name: "RewardType",
        variant: "block",
      },
      Token::Str("value"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::StructEnd,
    ];

    let tokens_d: &[Token] = &[
      Token::Struct {
        name: "TraceOutput",
        len: 4,
      },
      Token::Str("type"),
      Token::UnitVariant {
        name: "TraceCategory",
        variant: "suicide",
      },
      Token::Str("subtraces"),
      Token::U64(0),
      Token::Str("traceAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("action"),
      Token::Struct {
        name: "TraceActionSuicide",
        len: 3,
      },
      Token::Str("address"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("refundAddress"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("balance"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
    assert_tokens(&c.readable(), tokens_c);
    assert_tokens(&d.readable(), tokens_d);
  }
}
