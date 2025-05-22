use ueb3::api::NsOts as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eip::eip1898::BlockId;
use ueb3::types::eth::Address;
use ueb3::types::eth::Bytes;
use ueb3::types::eth::H256;
use ueb3::types::rpc::ots::BlockDetails;
use ueb3::types::rpc::ots::BlockTransactions;
use ueb3::types::rpc::ots::BlockTransactionSearch;
use ueb3::types::rpc::ots::ContractCreator;
use ueb3::types::rpc::ots::Operation;
use ueb3::types::rpc::ots::TransactionTrace;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  let value: u64 = transport.api_level()?;
  println!("[ots_getApiLevel]: {value:?}");

  let input: H256 = "0x9483cb44bd1bcf06a75cab29cf43e838d31af5d76661967407425a49c9945fbc".parse()?;
  let value: Vec<Operation> = transport.internal_operations(input)?;
  println!("[ots_getInternalOperations]: {value:#?}");

  let input: Address = "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD".parse()?;
  let block: BlockId = 17_854_438.into();
  let value: bool = transport.has_code(&input, Some(block))?;
  println!("[ots_hasCode]: {value:?}");

  let input: H256 = "0xcd7c73176e48f01bc7c3f563f674302a3421834e4bc29c2c182125dded44b377".parse()?;
  let value: Option<Bytes> = transport.transaction_error(input)?;
  println!("[ots_getTransactionError]: {value:?}");

  let input: H256 = "0xc6a5e4a4cb6ffd59a041f6683fb9d464be6d167fc3b6533671b3ea68ac4febac".parse()?;
  let value: Vec<TransactionTrace> = transport.trace_transaction(input)?;
  println!("[ots_traceTransaction]: {value:#?}");

  let input: BlockId = 17_854_439.into();
  let value: Option<BlockDetails> = transport.block_details(input)?;
  println!("[ots_getBlockDetails]: {value:#?}");

  let input: BlockId = "0x0c14a6b05dcd7de00cb87ba149692a49d13593a93563738ec2e2993653191b8c".parse()?;
  let value: Option<BlockDetails> = transport.block_details(input)?;
  println!("[ots_getBlockDetailsByHash]: {value:#?}");

  let input: u64 = 17_854_439;
  let count: u8 = 25;
  let value: Option<BlockTransactions> = transport.block_transactions(input, 0, count)?;
  println!("[ots_getBlockTransactions]: {value:#?}");

  let input: Address = "0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990".parse()?;
  let block: u64 = 0;
  let count: u16 = 25;
  let value: BlockTransactionSearch = transport.search_transactions_before(&input, block, count)?;
  println!("[ots_searchTransactionsBefore]: {value:#?}");

  let input: Address = "0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990".parse()?;
  let block: u64 = 0;
  let count: u16 = 25;
  let value: BlockTransactionSearch = transport.search_transactions_after(&input, block, count)?;
  println!("[ots_searchTransactionsAfter]: {value:#?}");

  let input: Address = "0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990".parse()?;
  let nonce: u64 = 24;
  let value: Option<H256> = transport.transaction_by_sender_and_nonce(&input, nonce)?;
  println!("[ots_getTransactionBySenderAndNonce]: {value:?}");

  let input: Address = "0x9008D19f58AAbD9eD0D60971565AA8510560ab41".parse()?;
  let value: Option<ContractCreator> = transport.contract_creator(&input)?;
  println!("[ots_getContractCreator]: {value:#?}");

  Ok(())
}
