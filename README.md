# ueb3

A typed Ethereum JSON-RPC client for Rust.

ueb3 covers the standard `eth`, `net` and `web3` methods, plus the extra namespaces that node clients expose: `trace` (Parity-style traces), `txpool`, Erigon, Reth and Otterscan (`ots`). Requests and responses are plain Rust types, including every transaction type up to EIP-7702, and filters for new blocks, transactions or logs can be consumed as async streams.

## Example

```rust
use ueb3::api::Provider as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eth::H256;
use ueb3::ureq::Ureq;

fn main() -> Result<()> {
  let client = RpcClient::new(Ureq::new("https://ethereum-rpc.publicnode.com")?);

  // The first EIP-1559 transaction on mainnet.
  let hash: H256 = "0xed382cb554ad10e94921d263a56c670669d6c380bbdacdbf96fed625b7132a1d".parse()?;

  if let Some(receipt) = client.transaction_receipt(hash)? {
    println!("included in block {}", receipt.block_number);
  }

  Ok(())
}
```

More examples are in [`examples/`](examples). There is one per namespace, plus filters for blocks, transactions and logs, and a walk through the first transaction of each type. They use a public node by default; build with `--features node-local` to use `http://localhost:8545` instead.

```sh
cd examples
cargo run --example fork_transactions
```

## Crates

| Crate | What it does |
|---|---|
| `ueb3-api` | The `Provider` trait with one method per RPC call, the namespace extensions, filter streams and pending-transaction futures. |
| `ueb3-rpc` | JSON-RPC 2.0 requests and responses, single and batched. |
| `ueb3-types` | Ethereum types: hashes, addresses, quantities, blocks, transactions, receipts, logs, traces, and the EIPs they depend on (191, 1186, 1898, 2718, 2930, 4895, 7702). |
| `ueb3-serde` | Serde helpers for Ethereum's hex encodings. |
| `ueb3-ureq` | The blocking HTTP transport. |
| `ueb3-utils` | Shared macros and traits. |

`ueb3` re-exports them as `ueb3::api`, `ueb3::rpc` and so on.

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.

[ureq]: https://github.com/algesten/ureq
