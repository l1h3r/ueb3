use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct as _;

// -----------------------------------------------------------------------------
// Erigon Filter Count
// -----------------------------------------------------------------------------

/// The maximum number of log entries returned from the query.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum FilterCount {
  /// Maximum number of logs to return.
  Log(u64),
  /// Maximum number of blocks to scan for log entries.
  Block(u64),
}

impl Default for FilterCount {
  #[inline]
  fn default() -> Self {
    Self::Block(1)
  }
}

// -----------------------------------------------------------------------------
// Erigon Filter Options
// -----------------------------------------------------------------------------

/// A set of customization options for a transaction log filter object.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct FilterOptions {
  /// The maximum number of log entries returned from the query.
  pub count: FilterCount,
  /// Whether to match topics by inclusion with no consideration of order.
  pub ignore_topics_order: bool,
}

impl FilterOptions {
  /// Creates a new `FilterOptions` with `FilterCount::Log(count)`.
  #[inline]
  pub const fn new_log(count: u64, ignore_topics_order: bool) -> Self {
    Self {
      count: FilterCount::Log(count),
      ignore_topics_order,
    }
  }

  /// Creates a new `FilterOptions` with `FilterCount::Block(count)`.
  #[inline]
  pub const fn new_block(count: u64, ignore_topics_order: bool) -> Self {
    Self {
      count: FilterCount::Block(count),
      ignore_topics_order,
    }
  }
}

impl Serialize for FilterOptions {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let fields: usize = 1 + usize::from(self.ignore_topics_order);
    let mut state: S::SerializeStruct = serializer.serialize_struct("FilterOptions", fields)?;

    match self.count {
      FilterCount::Log(ref inner) => {
        state.serialize_field("logCount", inner)?;
        state.skip_field("blockCount")?;
      }
      FilterCount::Block(ref inner) => {
        state.skip_field("logCount")?;
        state.serialize_field("blockCount", inner)?;
      }
    }

    if self.ignore_topics_order {
      state.serialize_field("ignoreTopicsOrder", &true)?;
    } else {
      state.skip_field("ignoreTopicsOrder")?;
    }

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

  use crate::rpc::erigon::FilterCount;
  use crate::rpc::erigon::FilterOptions;

  #[test]
  fn test_serde() {
    let a: FilterOptions = FilterOptions::default();

    let b: FilterOptions = FilterOptions {
      count: FilterCount::Log(1000),
      ignore_topics_order: true,
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "FilterOptions",
        len: 1,
      },
      Token::Str("blockCount"),
      Token::U64(1),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "FilterOptions",
        len: 2,
      },
      Token::Str("logCount"),
      Token::U64(1000),
      Token::Str("ignoreTopicsOrder"),
      Token::Bool(true),
      Token::StructEnd,
    ];

    assert_ser_tokens(&a.readable(), tokens_a);
    assert_ser_tokens(&b.readable(), tokens_b);
  }
}
