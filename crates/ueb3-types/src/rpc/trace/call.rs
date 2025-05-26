use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::H256;
use crate::rpc::trace::StateDiffAccount;
use crate::rpc::trace::TraceOutput;
use crate::rpc::trace::VmTrace;

// -----------------------------------------------------------------------------
// Trace Call Result
// -----------------------------------------------------------------------------

/// The result of an ad-hoc tracing RPC call.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceCallResult {
  /// The raw output data of the trace.
  pub output: Bytes,
  /// The account transitions caused by the transaction.
  pub state_diff: Option<BTreeMap<Address, StateDiffAccount>>,
  /// A list of transaction trace objects.
  pub trace: Vec<TraceOutput>,
  /// The VM state trace throughout the execution of the transaction.
  pub vm_trace: Option<VmTrace>,
  /// The transaction hash.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub transaction_hash: Option<H256>,
}

impl Debug for TraceCallResult {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TraceCallResult")
      .field("output", &self.output)
      .field("state_diff", &self.state_diff.format())
      .field("trace", &self.trace)
      .field("vm_trace", &self.vm_trace.format())
      .field("transaction_hash", &self.transaction_hash.format())
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Trace Type
// -----------------------------------------------------------------------------

/// A trace type identifier.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum TraceType {
  /// `vmTrace` trace type.
  VmTrace,
  /// `trace` trace type.
  Trace,
  /// `stateDiff` trace type.
  StateDiff,
}

impl TraceType {
  /// An array of all `TraceType` values.
  pub const ALL: [Self; 3] = [Self::VmTrace, Self::Trace, Self::StateDiff];
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::TraceCallResult;
  use crate::rpc::trace::TraceType;

  #[test]
  fn test_serde_trace_call_result() {
    const SEND: &str = "0x8bbb73bcb5d553b5a556358d27625323fd781d37";
    const HASH: &str = "0x17104ac9d3312d8c136b7f44d4b8b47852618065ebfa534bd2d3b5ef218ca1f3";

    let a: TraceCallResult = TraceCallResult::default();

    let b: TraceCallResult = TraceCallResult {
      output: Default::default(),
      state_diff: Some([(SEND.parse().unwrap(), Default::default())].into()),
      trace: Vec::new(),
      vm_trace: Some(Default::default()),
      transaction_hash: Some(HASH.parse().unwrap()),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceCallResult",
        len: 4,
      },
      Token::Str("output"),
      Token::Str("0x"),
      Token::Str("stateDiff"),
      Token::None,
      Token::Str("trace"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("vmTrace"),
      Token::None,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceCallResult",
        len: 5,
      },
      Token::Str("output"),
      Token::Str("0x"),
      Token::Str("stateDiff"),
      Token::Some,
      Token::Map { len: Some(1) },
      Token::Str(SEND),
      Token::Struct {
        name: "StateDiffAccount",
        len: 4,
      },
      Token::Str("balance"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("code"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("nonce"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("storage"),
      Token::Map { len: Some(0) },
      Token::MapEnd,
      Token::StructEnd,
      Token::MapEnd,
      Token::Str("trace"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("vmTrace"),
      Token::Some,
      Token::Struct {
        name: "VmTrace",
        len: 2,
      },
      Token::Str("code"),
      Token::Str("0x"),
      Token::Str("ops"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
      Token::Str("transactionHash"),
      Token::Some,
      Token::Str(HASH),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }

  #[test]
  fn test_serde_trace_type() {
    assert_tokens(
      &TraceType::VmTrace,
      &[Token::UnitVariant {
        name: "TraceType",
        variant: "vmTrace",
      }],
    );

    assert_tokens(
      &TraceType::Trace,
      &[Token::UnitVariant {
        name: "TraceType",
        variant: "trace",
      }],
    );

    assert_tokens(
      &TraceType::StateDiff,
      &[Token::UnitVariant {
        name: "TraceType",
        variant: "stateDiff",
      }],
    );
  }
}
