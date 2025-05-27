use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::ser::SerializeStruct as _;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten as _;
use ueb3_serde::maybe::Maybe;

use crate::eth::H256;
use crate::rpc::RawValue;
use crate::rpc::trace::TraceCategory;
use crate::rpc::trace::TraceHeader;
use crate::rpc::trace::TraceOutput;
use crate::rpc::trace::TracePayload;

// -----------------------------------------------------------------------------
// Parity Trace Object
// -----------------------------------------------------------------------------

/// A transaction trace object.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ParityTrace {
  /// The common trace properties.
  pub header: TraceHeader,
  /// The type-specific trace properties.
  pub output: TraceOutput,
}

impl Debug for ParityTrace {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("ParityTrace")
      .field("type", &self.output.kind())
      .field("header", &self.header)
      .field("output", &self.output)
      .finish()
  }
}

impl<'de> Deserialize<'de> for ParityTrace {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl<'de> Visitor<'de> for __Visitor {
      type Value = ParityTrace;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a parity-style trace object")
      }

      fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        #[derive(Deserialize)]
        #[serde(variant_identifier)]
        enum Field {
          #[serde(rename = "type")]
          Type,
          #[serde(rename = "blockHash")]
          BlockHash,
          #[serde(rename = "blockNumber")]
          BlockNumber,
          #[serde(rename = "transactionHash")]
          TransactionHash,
          #[serde(rename = "transactionPosition")]
          TransactionPosition,
          #[serde(rename = "subtraces")]
          Subtraces,
          #[serde(rename = "traceAddress")]
          TraceAddress,
          #[serde(rename = "action")]
          Action,
          #[serde(rename = "result")]
          Result,
          #[serde(rename = "error")]
          Error,
        }

        let mut category: Maybe<A, TraceCategory> = Maybe::NONE;
        let mut block_hash: Maybe<A, Option<H256>> = Maybe::NONE;
        let mut block_number: Maybe<A, Option<u64>> = Maybe::NONE;
        let mut hash: Maybe<A, Option<H256>> = Maybe::NONE;
        let mut index: Maybe<A, Option<u64>> = Maybe::NONE;

        let mut subtraces: Maybe<A, u64> = Maybe::NONE;
        let mut trace_address: Maybe<A, Vec<u64>> = Maybe::NONE;
        let mut action: Maybe<A, &'de RawValue> = Maybe::NONE;
        let mut result: Maybe<A, &'de RawValue> = Maybe::NONE;
        let mut error: Maybe<A, String> = Maybe::NONE;

        while let Some(field) = access.next_key()? {
          match field {
            Field::Type => category.set(&mut access, "type")?,
            // -----------------------------------------------------------------
            // Header Fields
            // -----------------------------------------------------------------
            Field::BlockHash => block_hash.set(&mut access, "blockHash")?,
            Field::BlockNumber => block_number.set(&mut access, "blockNumber")?,
            Field::TransactionHash => hash.set(&mut access, "transactionHash")?,
            Field::TransactionPosition => index.set(&mut access, "transactionPosition")?,
            // -----------------------------------------------------------------
            // Output Fields
            // -----------------------------------------------------------------
            Field::Subtraces => subtraces.set(&mut access, "subtraces")?,
            Field::TraceAddress => trace_address.set(&mut access, "traceAddress")?,
            Field::Action => action.set(&mut access, "action")?,
            Field::Result => result.set(&mut access, "result")?,
            Field::Error => error.set(&mut access, "error")?,
          }
        }

        let header: TraceHeader = TraceHeader {
          block_hash: block_hash.get_required("blockHash")?,
          block_number: block_number.get_required("blockNumber")?,
          transaction_hash: hash.get_required("transactionHash")?,
          transaction_position: index.get_required("transactionPosition")?,
        };

        let category: TraceCategory = category.get_required("type")?;
        let subtraces: u64 = subtraces.get_required("subtraces")?;
        let trace_address: Vec<u64> = trace_address.get_required("traceAddress")?;
        let action: &RawValue = action.get_required("action")?;
        let result: &RawValue = result.get_optional().unwrap_or(RawValue::NULL);
        let error: Option<String> = error.get_optional();

        let output: TraceOutput = match category {
          TraceCategory::Call => TraceOutput::Call(TracePayload {
            subtraces,
            trace_address,
            action: Deserialize::deserialize(action).map_err(A::Error::custom)?,
            result: Deserialize::deserialize(result).map_err(A::Error::custom)?,
            error,
          }),
          TraceCategory::Create => TraceOutput::Create(TracePayload {
            subtraces,
            trace_address,
            action: Deserialize::deserialize(action).map_err(A::Error::custom)?,
            result: Deserialize::deserialize(result).map_err(A::Error::custom)?,
            error,
          }),
          TraceCategory::Reward => TraceOutput::Reward(TracePayload {
            subtraces,
            trace_address,
            action: Deserialize::deserialize(action).map_err(A::Error::custom)?,
            result: Deserialize::deserialize(result).map_err(A::Error::custom)?,
            error,
          }),
          TraceCategory::Suicide => TraceOutput::Suicide(TracePayload {
            subtraces,
            trace_address,
            action: Deserialize::deserialize(action).map_err(A::Error::custom)?,
            result: Deserialize::deserialize(result).map_err(A::Error::custom)?,
            error,
          }),
        };

        Ok(ParityTrace { header, output })
      }
    }

    deserializer.deserialize_map(__Visitor)
  }
}

impl Serialize for ParityTrace {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let fields: usize = self.header.fields() + self.output.fields();
    let mut state: S::SerializeStruct = serializer.serialize_struct("ParityTrace", fields)?;

    self.header.serialize_fields(&mut state)?;
    self.output.serialize_fields(&mut state)?;

    state.end()
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_ser_tokens;

  use crate::rpc::trace::ParityTrace;
  use crate::rpc::trace::TraceOutput;

  #[test]
  fn test_serde() {
    let a: ParityTrace = ParityTrace {
      header: Default::default(),
      output: TraceOutput::Call(Default::default()),
    };

    let b: ParityTrace = ParityTrace {
      header: Default::default(),
      output: TraceOutput::Create(Default::default()),
    };

    let c: ParityTrace = ParityTrace {
      header: Default::default(),
      output: TraceOutput::Reward(Default::default()),
    };

    let d: ParityTrace = ParityTrace {
      header: Default::default(),
      output: TraceOutput::Suicide(Default::default()),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "ParityTrace",
        len: 8,
      },
      Token::Str("blockHash"),
      Token::None,
      Token::Str("blockNumber"),
      Token::None,
      Token::Str("transactionHash"),
      Token::None,
      Token::Str("transactionPosition"),
      Token::None,
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
        name: "ParityTrace",
        len: 8,
      },
      Token::Str("blockHash"),
      Token::None,
      Token::Str("blockNumber"),
      Token::None,
      Token::Str("transactionHash"),
      Token::None,
      Token::Str("transactionPosition"),
      Token::None,
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
        name: "ParityTrace",
        len: 8,
      },
      Token::Str("blockHash"),
      Token::None,
      Token::Str("blockNumber"),
      Token::None,
      Token::Str("transactionHash"),
      Token::None,
      Token::Str("transactionPosition"),
      Token::None,
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
        name: "ParityTrace",
        len: 8,
      },
      Token::Str("blockHash"),
      Token::None,
      Token::Str("blockNumber"),
      Token::None,
      Token::Str("transactionHash"),
      Token::None,
      Token::Str("transactionPosition"),
      Token::None,
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

    assert_ser_tokens(&a.readable(), tokens_a);
    assert_ser_tokens(&b.readable(), tokens_b);
    assert_ser_tokens(&c.readable(), tokens_c);
    assert_ser_tokens(&d.readable(), tokens_d);
  }
}
