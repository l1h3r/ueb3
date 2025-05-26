use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::Address;

// -----------------------------------------------------------------------------
// Trace Filter Mode
// -----------------------------------------------------------------------------

/// The mode of matching `from/to_address` in a filter request.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum TraceFilterMode {
  /// Match addresses when included in `from_address` OR `to_address`.
  #[default]
  Union,
  /// Match addresses when included in `from_address` AND `to_address`.
  Intersection,
}

// -----------------------------------------------------------------------------
// Trace Filter Request
// -----------------------------------------------------------------------------

/// A trace filter request object.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceFilterRequest {
  /// The block number lower bound.
  #[serde(with = "ueb3_serde::quantity::opt")]
  pub from_block: Option<u64>,
  /// The block number upper bound.
  #[serde(with = "ueb3_serde::quantity::opt")]
  pub to_block: Option<u64>,
  /// Only include traces sent from these addresses.
  #[serde(default)]
  pub from_address: Vec<Address>,
  /// Only include traces sent to these addresses.
  #[serde(default)]
  pub to_address: Vec<Address>,
  /// The address matching mode.
  #[serde(default)]
  pub mode: TraceFilterMode,
  /// The offset of the results returned.
  pub after: Option<u64>,
  /// The maximum number of results returned.
  pub count: Option<u64>,
}

impl Debug for TraceFilterRequest {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TraceFilterRequest")
      .field("from_block", &self.from_block.format())
      .field("to_block", &self.to_block.format())
      .field("from_address", &self.from_address)
      .field("to_address", &self.to_address)
      .field("mode", &self.mode)
      .field("after", &self.after.format())
      .field("count", &self.count.format())
      .finish()
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

  use crate::rpc::trace::TraceFilterMode;
  use crate::rpc::trace::TraceFilterRequest;

  #[test]
  fn test_serde() {
    const RECV: &str = "0x8bbb73bcb5d553b5a556358d27625323fd781d37";

    let a: TraceFilterRequest = TraceFilterRequest::default();

    let b: TraceFilterRequest = TraceFilterRequest {
      from_block: Some(0x2ed0c4),
      to_block: Some(0x2ed128),
      from_address: Vec::new(),
      to_address: vec![RECV.parse().unwrap()],
      mode: TraceFilterMode::Intersection,
      after: Some(1000),
      count: Some(100),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceFilterRequest",
        len: 7,
      },
      Token::Str("fromBlock"),
      Token::None,
      Token::Str("toBlock"),
      Token::None,
      Token::Str("fromAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("toAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("mode"),
      Token::UnitVariant {
        name: "TraceFilterMode",
        variant: "union",
      },
      Token::Str("after"),
      Token::None,
      Token::Str("count"),
      Token::None,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceFilterRequest",
        len: 7,
      },
      Token::Str("fromBlock"),
      Token::Some,
      Token::Str("0x2ed0c4"),
      Token::Str("toBlock"),
      Token::Some,
      Token::Str("0x2ed128"),
      Token::Str("fromAddress"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("toAddress"),
      Token::Seq { len: Some(1) },
      Token::Str(RECV),
      Token::SeqEnd,
      Token::Str("mode"),
      Token::UnitVariant {
        name: "TraceFilterMode",
        variant: "intersection",
      },
      Token::Str("after"),
      Token::Some,
      Token::U64(1000),
      Token::Str("count"),
      Token::Some,
      Token::U64(100),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
