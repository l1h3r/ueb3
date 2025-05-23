use std::collections::HashMap;
use ueb3::api::NsErigon as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eip::eip1898::BlockId;
use ueb3::types::eth::Address;
use ueb3::types::eth::H256;
use ueb3::types::eth::Wei;
use ueb3::types::rpc::Block;
use ueb3::types::rpc::BlockNumber;
use ueb3::types::rpc::erigon::FilterOptions;
use ueb3::types::rpc::erigon::Forks;
use ueb3::types::rpc::Filter;
use ueb3::types::rpc::Transaction;
use ueb3::types::rpc::TransactionLog;
use ueb3::types::rpc::TransactionReceipt;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  let value: u64 = transport.block_number(None)?;
  println!("[erigon_blockNumber]: {value:?}");

  let value: u64 = transport.block_number(Some(BlockNumber::Earliest))?;
  println!("[erigon_blockNumber(earliest)]: {value}");

  let value: u64 = transport.block_number(Some(BlockNumber::Latest))?;
  println!("[erigon_blockNumber(latest)]: {value}");

  let value: u64 = transport.block_number(Some(BlockNumber::Safe))?;
  println!("[erigon_blockNumber(safe)]: {value}");

  let value: u64 = transport.block_number(Some(BlockNumber::Finalized))?;
  println!("[erigon_blockNumber(finalized)]: {value}");

  let value: u64 = transport.block_number(Some(BlockNumber::Pending))?;
  println!("[erigon_blockNumber(pending)]: {value}");

  let value: Forks = transport.forks()?;
  println!("[erigon_forks]: {value:#?}");

  let input: BlockId = "0x4e3a3754410177e6937ef1f84bba68ea139e8d1a2258c5f85db9f1cd715a1bdd".parse()?;
  let value: HashMap<Address, Wei> = transport.balance_changes_in_block(input)?;
  println!("[erigon_getBalanceChangesInBlock]: {value:#?}");

  let input: u64 = 0x55ba4224;
  let value: Block<H256> = transport.block_summary_by_timestamp(input)?;
  println!("[erigon_getBlockByTimestamp(.., false)]: {value:#?}");

  let input: u64 = 0x55ba4224;
  let value: Block<Transaction> = transport.block_details_by_timestamp(input)?;
  println!("[erigon_getBlockByTimestamp(.., true)]: {value:#?}");

  let input: H256 = "0xa917fcc721a5465a484e9be17cda0cc5493933dd3bc70c9adbee192cb419c9d7".parse()?;
  let value: Vec<TransactionReceipt> = transport.block_receipts_by_block_hash(input)?;
  println!("[erigon_getBlockReceiptsByBlockHash]: {value:#?}");

  let input: BlockId = "0xa917fcc721a5465a484e9be17cda0cc5493933dd3bc70c9adbee192cb419c9d7".parse()?;
  let value: Block<()> = transport.header(input)?;
  println!("[erigon_getHeaderByHash]: {value:#?}");

  let input: BlockId = "0x10c582e".parse()?;
  let value: Block<()> = transport.header(input)?;
  println!("[erigon_getHeaderByNumber]: {value:#?}");

  let input: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
  let input: Filter = Filter::new().with_address(input);
  let f_opt: FilterOptions = FilterOptions::new_log(1000, true);
  let value: Vec<TransactionLog> = transport.latest_logs(&input, f_opt)?;
  println!("[erigon_getLatestLogs]: {value:#?}");

  let input: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
  let input: Filter = Filter::new().with_address(input);
  let value: Vec<TransactionLog> = transport.logs(&input)?;
  println!("[erigon_getLogs]: {value:#?}");

  let input: H256 = "0x3f07a9c83155594c000642e7d60e8a8a00038d03e9849171a05ed0e2d47acbb3".parse()?;
  let value: Vec<TransactionLog> = transport.logs_by_hash(input)?;
  println!("[erigon_getLogsByHash]: {value:#?}");

  Ok(())
}
