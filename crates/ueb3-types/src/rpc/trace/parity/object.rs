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
        let mut block_hash: Maybe<A, H256> = Maybe::NONE;
        let mut block_number: Maybe<A, u64> = Maybe::NONE;
        let mut hash: Maybe<A, H256> = Maybe::NONE;
        let mut index: Maybe<A, u64> = Maybe::NONE;

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
          block_hash: block_hash.get_optional(),
          block_number: block_number.get_optional(),
          transaction_hash: hash.get_optional(),
          transaction_position: index.get_optional(),
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
    let fields: usize = 1 + self.header.fields() + self.output.fields();
    let mut state: S::SerializeStruct = serializer.serialize_struct("ParityTrace", fields)?;

    state.serialize_field("type", &self.output.kind())?;

    self.header.serialize_fields(&mut state)?;
    self.output.serialize_fields(&mut state)?;

    state.end()
  }
}
