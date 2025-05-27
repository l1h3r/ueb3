use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::Bytes;
use crate::eth::U256;

// -----------------------------------------------------------------------------
// Vm Trace
// -----------------------------------------------------------------------------

/// A trace object containing a full VM operation.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VmTrace {
  /// The code associated with the operation.
  pub code: Bytes,
  /// The instructions executed for the operation.
  pub ops: Vec<VmTraceOp>,
}

// -----------------------------------------------------------------------------
// Vm Trace Op
// -----------------------------------------------------------------------------

/// A trace object containing a VM operation.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VmTraceOp {
  /// The total amount of gas used for this operation.
  pub cost: u64,
  /// The VM operation execution record.
  pub ex: Option<VmTraceEx>,
  /// The program counter.
  pub pc: u64,
  /// The subtrace.
  pub sub: Option<VmTrace>,
  /// The opcode name.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub op: Option<String>,
  /// The index of the instruction.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub idx: Option<String>,
}

impl Debug for VmTraceOp {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("VmTraceOp")
      .field("cost", &self.cost)
      .field("ex", &self.ex.format())
      .field("pc", &self.pc)
      .field("sub", &self.sub.format())
      .field("op", &self.op.format())
      .field("idx", &self.idx.format())
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Vm Trace Ex
// -----------------------------------------------------------------------------

/// A trace object containing a VM operation execution.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VmTraceEx {
  /// The memory change.
  pub mem: Option<VmTraceMem>,
  /// The value added to the stack.
  pub push: Vec<U256>,
  /// The storage change.
  pub store: Option<VmTraceStore>,
  /// The total amount of gas used.
  pub used: u64,
}

impl Debug for VmTraceEx {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("VmTraceEx")
      .field("mem", &self.mem.format())
      .field("push", &self.push)
      .field("store", &self.store.format())
      .field("used", &self.used)
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Vm Trace Mem
// -----------------------------------------------------------------------------

/// A trace object containing a memory change.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VmTraceMem {
  /// A copy of the accessed memory.
  pub data: Bytes,
  /// The memory offset of the change.
  pub off: u64,
}

// -----------------------------------------------------------------------------
// Vm Trace Store
// -----------------------------------------------------------------------------

/// A trace object containing a storage change.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VmTraceStore {
  /// The storage key.
  pub key: U256,
  /// The storage value.
  pub val: U256,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eth::U256;
  use crate::rpc::trace::VmTrace;
  use crate::rpc::trace::VmTraceEx;
  use crate::rpc::trace::VmTraceMem;
  use crate::rpc::trace::VmTraceOp;
  use crate::rpc::trace::VmTraceStore;

  #[test]
  fn test_serde() {
    const DATA: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";

    const KEY: &str = "0x98af170d2fe1f73c638d070776fb485cc93ce983958d37cb72397982fdb58b41";
    const VAL: &str = "0x588a27e6";

    let a: VmTrace = VmTrace::default();

    let b: VmTrace = VmTrace {
      code: Default::default(),
      ops: vec![VmTraceOp {
        cost: 82261,
        ex: Some(VmTraceEx {
          mem: Some(VmTraceMem {
            data: DATA.parse().unwrap(),
            off: 96,
          }),
          push: vec![U256::from(1)],
          store: Some(VmTraceStore {
            key: KEY.parse().unwrap(),
            val: VAL.parse().unwrap(),
          }),
          used: 105521,
        }),
        pc: 124,
        sub: Some(VmTrace {
          code: Default::default(),
          ops: vec![VmTraceOp {
            cost: 3,
            ex: Some(VmTraceEx {
              mem: None,
              push: vec![U256::from(96)],
              store: None,
              used: 81558,
            }),
            pc: 0,
            sub: None,
            op: Some("PUSH1".into()),
            idx: Some("3-41-0".into()),
          }],
        }),
        op: Some("CALL".into()),
        idx: Some("3-41".into()),
      }],
    };

    let tokens_a: &[Token] = &[
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
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "VmTrace",
        len: 2,
      },
      Token::Str("code"),
      Token::Str("0x"),
      Token::Str("ops"),
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "VmTraceOp",
        len: 6,
      },
      Token::Str("cost"),
      Token::U64(82261),
      Token::Str("ex"),
      Token::Some,
      Token::Struct {
        name: "VmTraceEx",
        len: 4,
      },
      Token::Str("mem"),
      Token::Some,
      Token::Struct {
        name: "VmTraceMem",
        len: 2,
      },
      Token::Str("data"),
      Token::Str(DATA),
      Token::Str("off"),
      Token::U64(96),
      Token::StructEnd,
      Token::Str("push"),
      Token::Seq { len: Some(1) },
      Token::Str("0x1"),
      Token::SeqEnd,
      Token::Str("store"),
      Token::Some,
      Token::Struct {
        name: "VmTraceStore",
        len: 2,
      },
      Token::Str("key"),
      Token::Str(KEY),
      Token::Str("val"),
      Token::Str(VAL),
      Token::StructEnd,
      Token::Str("used"),
      Token::U64(105521),
      Token::StructEnd,
      Token::Str("pc"),
      Token::U64(124),
      Token::Str("sub"),
      Token::Some,
      Token::Struct {
        name: "VmTrace",
        len: 2,
      },
      Token::Str("code"),
      Token::Str("0x"),
      Token::Str("ops"),
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "VmTraceOp",
        len: 6,
      },
      Token::Str("cost"),
      Token::U64(3),
      Token::Str("ex"),
      Token::Some,
      Token::Struct {
        name: "VmTraceEx",
        len: 4,
      },
      Token::Str("mem"),
      Token::None,
      Token::Str("push"),
      Token::Seq { len: Some(1) },
      Token::Str("0x60"),
      Token::SeqEnd,
      Token::Str("store"),
      Token::None,
      Token::Str("used"),
      Token::U64(81558),
      Token::StructEnd,
      Token::Str("pc"),
      Token::U64(0),
      Token::Str("sub"),
      Token::None,
      Token::Str("op"),
      Token::Some,
      Token::Str("PUSH1"),
      Token::Str("idx"),
      Token::Some,
      Token::Str("3-41-0"),
      Token::StructEnd,
      Token::SeqEnd,
      Token::StructEnd,
      Token::Str("op"),
      Token::Some,
      Token::Str("CALL"),
      Token::Str("idx"),
      Token::Some,
      Token::Str("3-41"),
      Token::StructEnd,
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
