use ueb3::api::Provider as _;
use ueb3::rpc::Error;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eip::eip1186::AccountProof;
use ueb3::types::eip::eip1898::BlockId;
use ueb3::types::eip::eip2718::TransactionType;
use ueb3::types::eip::eip2930::AccessListResult;
use ueb3::types::eth::Address;
use ueb3::types::eth::Bytes;
use ueb3::types::eth::H64;
use ueb3::types::eth::H256;
use ueb3::types::eth::Signature;
use ueb3::types::eth::Slice;
use ueb3::types::eth::U256;
use ueb3::types::eth::Wei;
use ueb3::types::rpc::Block;
use ueb3::types::rpc::BlockNumber;
use ueb3::types::rpc::FeeHistory;
use ueb3::types::rpc::Filter;
use ueb3::types::rpc::SyncResult;
use ueb3::types::rpc::Transaction;
use ueb3::types::rpc::TransactionId;
use ueb3::types::rpc::TransactionLog;
use ueb3::types::rpc::TransactionReceipt;
use ueb3::types::rpc::TransactionRequest;
use ueb3::types::rpc::WorkBundle;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

static FILTER: &str = "See examples/filter/*.rs";

fn transaction_request(transport: &RpcClient) -> Result<TransactionRequest> {
  let mut request: TransactionRequest = TransactionRequest {
    chain_id: None,
    transaction_type: Some(TransactionType::Eip2930),
    from: None,
    to: None,
    nonce: None,
    gas_limit: None,
    gas_price: None,
    max_priority_fee_per_gas: None,
    max_fee_per_gas: None,
    max_fee_per_blob_gas: None,
    value: None,
    input: None,
    access_list: None,
    blob_versioned_hashes: None,
    authorization_list: None,
    ..Default::default()
  };

  // Mnemonic: test test test test test test test test test test test junk
  let send: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;
  let recv: Address = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".parse()?;

  request.chain_id = Some(transport.chain_id()?);
  request.from = Some(send);
  request.to = Some(recv);
  request.nonce = Some(transport.transaction_count(&send, None)? + 1);
  request.gas_limit = Some(transport.estimate_gas(&request, None)?);
  request.gas_price = Some(transport.gas_price()?);
  request.value = Some(U256::from(10_000_000_000_000_000_000_u64).into()); // 10 ETH
  request.access_list = Some(transport.create_access_list(&request, None)?.access_list);

  Ok(request)
}

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);
  let request: TransactionRequest = transaction_request(&transport)?;

  let value: Result<Vec<Address>> = transport.accounts();
  let value: Result<Vec<Address>, String> = value.map_err(|error| error.to_string());
  println!("[eth_accounts]: {value:?}");

  let value: Wei = transport.blob_base_fee()?;
  println!("[eth_blobBaseFee]: {value}");

  let value: u64 = transport.block_number()?;
  println!("[eth_blockNumber]: {value}");

  let input: &TransactionRequest = &request;
  let block: BlockId = BlockId::latest();
  let value: Bytes = transport.call(input, Some(block))?;
  println!("[eth_call]: {value}");

  let value: u64 = transport.chain_id()?;
  println!("[eth_chainId]: {value}");

  let value: Result<Address> = transport.coinbase();
  let value: Result<Address, String> = value.map_err(|error| error.to_string());
  println!("[eth_coinbase]: {value:?}");

  let input: &str = "(returnlll(suicide(caller)))";
  let value: Result<Bytes> = transport.compile_lll(input);
  let value: Result<Bytes, String> = value.map_err(|error| error.to_string());
  println!("[eth_compileLLL]: {value:?}");

  let input: &str = "0x1234";
  let value: Result<Bytes> = transport.compile_serpent(input);
  let value: Result<Bytes, String> = value.map_err(|error| error.to_string());
  println!("[eth_compileSerpent]: {value:?}");

  let input: &str = "contracttest { function multiply(uint a) returns(uint d) { return a * 7; } }";
  let value: Result<_> = transport.compile_solidity(input);
  let value: Result<_, String> = value.map_err(|error| error.to_string());
  println!("[eth_compileSolidity]: {value:?}");

  let input: &TransactionRequest = &request;
  let block: BlockId = BlockId::latest();
  let value: AccessListResult = transport.create_access_list(input, Some(block))?;
  println!("[eth_createAccessList]: {value:#?}");

  let input: &TransactionRequest = &request;
  let block: BlockId = BlockId::latest();
  let value: U256 = transport.estimate_gas(input, Some(block))?;
  println!("[eth_estimateGas]: {value}");

  let count: u64 = 0x4;
  let block: BlockNumber = BlockNumber::Latest;
  let percentiles: &[f64] = &[25.0, 75.0];
  let value: FeeHistory = transport.fee_history(count, block, percentiles)?;
  println!("[eth_feeHistory]: {value:#?}");

  let value: Wei = transport.gas_price()?;
  println!("[eth_gasPrice]: {value}");

  let input: Address = "0x8D97689C9818892B700e27F316cc3E41e17fBeb9".parse()?;
  let block: BlockId = BlockId::latest();
  let value: Wei = transport.balance(&input, Some(block))?;
  println!("[eth_getBalance]: {value}");

  let input: BlockId = "0x3f07a9c83155594c000642e7d60e8a8a00038d03e9849171a05ed0e2d47acbb3".parse()?;
  let value: Option<Block<H256>> = transport.block_summary(input)?;
  println!("[eth_getBlockByHash]: {value:#?}");

  let input: BlockId = "0xc5043f".parse()?;
  let value: Option<Block<H256>> = transport.block_summary(input)?;
  println!("[eth_getBlockByNumber]: {value:#?}");

  let input: BlockId = "0xc5043f".parse()?;
  let value: Option<Vec<TransactionReceipt>> = transport.block_receipts(input)?;
  println!("[eth_getBlockReceipts]: {value:#?}");

  let input: BlockId = "0x829df9bb801fc0494abf2f443423a49ffa32964554db71b098d332d87b70a48b".parse()?;
  let value: Option<u64> = transport.block_transaction_count(input)?;
  println!("[eth_getBlockTransactionCountByHash]: {value:?}");

  let input: BlockId = "0xc5043f".parse()?;
  let value: Option<u64> = transport.block_transaction_count(input)?;
  println!("[eth_getBlockTransactionCountByNumber]: {value:?}");

  let input: Address = "0x5B56438000bAc5ed2c6E0c1EcFF4354aBfFaf889".parse()?;
  let block: BlockId = BlockId::latest();
  let value: Bytes = transport.code(&input, Some(block))?;
  println!("[eth_getCode]: {value}");

  let value: Result<Vec<String>> = transport.compilers();
  let value: Result<Vec<String>, String> = value.map_err(|error| error.to_string());
  println!("[eth_getCompilers]: {value:?}");

  println!("[eth_getFilterChanges]: {FILTER}");
  println!("[eth_getFilterLogs]: {FILTER}");

  let scope: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
  let input: Filter = Filter::new().with_address(scope);
  let value: Vec<TransactionLog> = transport.logs(&input)?;
  println!("[eth_getLogs]: {value:#?}");

  let address: Address = "0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8".parse()?;
  let storage: &[H256] = &["0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".parse()?];
  let block: BlockId = BlockId::latest();
  let value: AccountProof = transport.proof(&address, storage, Some(block))?;
  println!("[eth_getProof]: {value:#?}");

  let input: Address = "0xE592427A0AEce92De3Edee1F18E0157C05861564".parse()?;
  let index: U256 = "0x0".parse().map_err(Error::other)?;
  let block: BlockId = BlockId::latest();
  let value: H256 = transport.storage(&input, index, Some(block))?;
  println!("[eth_getStorageAt]: {value}");

  let block: BlockId = "0x829df9bb801fc0494abf2f443423a49ffa32964554db71b098d332d87b70a48b".parse()?;
  let index: u64 = 0x0;
  let input: TransactionId = (block, index).into();
  let value: Option<Transaction> = transport.transaction(input)?;
  println!("[eth_getTransactionByBlockHashAndIndex]: {value:#?}");

  let block: BlockId = "0xc5043f".parse()?;
  let index: u64 = 0x0;
  let input: TransactionId = (block, index).into();
  let value: Option<Transaction> = transport.transaction(input)?;
  println!("[eth_getTransactionByBlockNumberAndIndex]: {value:#?}");

  let input: TransactionId = "0xb1fac2cb5074a4eda8296faebe3b5a3c10b48947dd9a738b2fdf859be0e1fbaf".parse()?;
  let value: Option<Transaction> = transport.transaction(input)?;
  println!("[eth_getTransactionByHash]: {value:#?}");

  let input: Address = "0x8D97689C9818892B700e27F316cc3E41e17fBeb9".parse()?;
  let block: BlockId = BlockId::latest();
  let value: u64 = transport.transaction_count(&input, Some(block))?;
  println!("[eth_getTransactionCount]: {value}");

  let input: H256 = "0x85d995eba9763907fdf35cd2034144dd9d53ce32cbec21349d4b12823c6860c5".parse()?;
  let value: Option<TransactionReceipt> = transport.transaction_receipt(input)?;
  println!("[eth_getTransactionReceipt]: {value:#?}");

  let input: BlockId = "0xa917fcc721a5465a484e9be17cda0cc5493933dd3bc70c9adbee192cb419c9d7".parse()?;
  let index: u64 = 0x0;
  let value: Option<Block<()>> = transport.uncle(input, index)?;
  println!("[eth_getUncleByBlockHashAndIndex]: {value:#?}");

  let input: BlockId = "0xc5043f".parse()?;
  let index: u64 = 0x0;
  let value: Option<Block<()>> = transport.uncle(input, index)?;
  println!("[eth_getUncleByBlockNumberAndIndex]: {value:#?}");

  let input: BlockId = "0xa917fcc721a5465a484e9be17cda0cc5493933dd3bc70c9adbee192cb419c9d7".parse()?;
  let value: Option<u64> = transport.uncle_count(input)?;
  println!("[eth_getUncleCountByBlockHash]: {value:?}");

  let input: BlockId = "0xc5043f".parse()?;
  let value: Option<u64> = transport.uncle_count(input)?;
  println!("[eth_getUncleCountByBlockNumber]: {value:?}");

  let value: Result<WorkBundle> = transport.work();
  let value: Result<WorkBundle, String> = value.map_err(|error| error.to_string());
  println!("[eth_getWork]: {value:?}");

  let value: Result<U256> = transport.hashrate();
  let value: Result<U256, String> = value.map_err(|error| error.to_string());
  println!("[eth_hashrate]: {value:?}");

  let value: Wei = transport.max_priority_fee()?;
  println!("[eth_maxPriorityFeePerGas]: {value}");

  let value: Result<bool> = transport.mining();
  let value: Result<bool, String> = value.map_err(|error| error.to_string());
  println!("[eth_mining]: {value:?}");

  println!("[eth_newBlockFilter]: {FILTER}");

  println!("[eth_newFilter]: {FILTER}");

  println!("[eth_newPendingTransactionFilter]: {FILTER}");

  let value: Result<String> = transport.protocol_version();
  let value: Result<String, String> = value.map_err(|error| error.to_string());
  println!("[eth_protocolVersion]: {value:?}");

  let input: Bytes = transport.sign_transaction(&request)?;
  let value: H256 = transport.send_raw_transaction(&input)?;
  println!("[eth_sendRawTransaction]: {value}");

  let input: &TransactionRequest = &request;
  let value: H256 = transport.send_transaction(&input)?;
  println!("[eth_sendTransaction]: {value:?}");

  // Mnemonic: test test test test test test test test test test test junk
  let input: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;
  let bytes: &Slice = "hello world".as_ref();
  let value: Signature = transport.sign(&input, bytes)?;
  println!("[eth_sign]: {value}");

  let input: &TransactionRequest = &request;
  let value: Bytes = transport.sign_transaction(input)?;
  println!("[eth_signTransaction]: {value}");

  let value: Result<bool> = transport.submit_hashrate(U256::zero(), H256::NULL);
  let value: Result<bool, String> = value.map_err(|error| error.to_string());
  println!("[eth_submitHashrate]: {value:?}");

  let value: Result<bool> = transport.submit_work(H64::NULL, H256::NULL, H256::NULL);
  let value: Result<bool, String> = value.map_err(|error| error.to_string());
  println!("[eth_submitWork]: {value:?}");

  let value: SyncResult<_> = transport.syncing()?;
  println!("[eth_syncing]: {value:#?}");

  println!("[eth_uninstallFilter]: {FILTER}");

  Ok(())
}
