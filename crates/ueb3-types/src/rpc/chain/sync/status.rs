use serde::Deserialize;
use serde::Serialize;

// -----------------------------------------------------------------------------
// Ethereum Node Sync Status
// -----------------------------------------------------------------------------

/// A snapshot of the current sync status of a node.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatus<T> {
  /// The block at which the node started syncing.
  #[serde(with = "ueb3_serde::quantity")]
  pub starting_block: u64,
  /// The current block, same as `eth_blockNumber`.
  #[serde(with = "ueb3_serde::quantity")]
  pub current_block: u64,
  /// An estimate of the highest block.
  #[serde(with = "ueb3_serde::quantity")]
  pub highest_block: u64,
  /// Additional data provided by the client.
  #[serde(flatten)]
  pub unknown_fields: T,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde::Deserialize;
  use serde::Serialize;
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::SyncStatus;

  #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
  #[serde(rename_all = "camelCase")]
  struct Extra {
    #[serde(with = "ueb3_serde::quantity")]
    warp_chunks_amount: u64,
    #[serde(with = "ueb3_serde::quantity")]
    warp_chunks_processed: u64,
  }

  #[test]
  fn test_serde() {
    let a: SyncStatus<()> = SyncStatus {
      starting_block: 0x0,
      current_block: 0x1518,
      highest_block: 0x9567a3,
      unknown_fields: (),
    };

    let b: SyncStatus<Extra> = SyncStatus {
      starting_block: 0x0,
      current_block: 0x1518,
      highest_block: 0x9567a3,
      unknown_fields: Extra {
        warp_chunks_amount: 0x7b,
        warp_chunks_processed: 0x1c8,
      },
    };

    let tokens_a: &[Token] = &[
      Token::Map { len: None },
      Token::Str("startingBlock"),
      Token::Str("0x0"),
      Token::Str("currentBlock"),
      Token::Str("0x1518"),
      Token::Str("highestBlock"),
      Token::Str("0x9567a3"),
      Token::MapEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Map { len: None },
      Token::Str("startingBlock"),
      Token::Str("0x0"),
      Token::Str("currentBlock"),
      Token::Str("0x1518"),
      Token::Str("highestBlock"),
      Token::Str("0x9567a3"),
      Token::Str("warpChunksAmount"),
      Token::Str("0x7b"),
      Token::Str("warpChunksProcessed"),
      Token::Str("0x1c8"),
      Token::MapEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
