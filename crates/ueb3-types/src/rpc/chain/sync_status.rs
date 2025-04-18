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

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::SyncStatus;

  #[test]
  fn test_serde() {
    let value: SyncStatus<()> = SyncStatus {
      starting_block: 0,
      current_block: 15,
      highest_block: 255,
      unknown_fields: (),
    };

    assert_tokens(
      &value.readable(),
      &[
        Token::Map { len: None },
        Token::Str("startingBlock"),
        Token::Str("0x0"),
        Token::Str("currentBlock"),
        Token::Str("0xf"),
        Token::Str("highestBlock"),
        Token::Str("0xff"),
        Token::MapEnd,
      ],
    );
  }
}
