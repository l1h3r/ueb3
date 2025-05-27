use std::collections::HashSet;
use ueb3::api::NsTrace as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eip::eip1898::BlockId;
use ueb3::types::eth::Bytes;
use ueb3::types::eth::H256;
use ueb3::types::rpc::trace::ParityTrace;
use ueb3::types::rpc::trace::TraceCallResult;
use ueb3::types::rpc::trace::TraceFilterRequest;
use ueb3::types::rpc::trace::TraceType;
use ueb3::types::rpc::TransactionRequest;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn filter() -> TraceFilterRequest {
  TraceFilterRequest {
    from_block: Some(0x0),
    to_block: Some(0x10),
    ..TraceFilterRequest::default()
  }
}

fn request() -> TransactionRequest {
  TransactionRequest {
      from: Some("0x407d73d8a49eeb85d32cf465507dd71d507100c1".parse().unwrap()),
      to: Some("0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b".parse().unwrap()),
      value: Some("0x186a0".parse().unwrap()),
    ..TransactionRequest::default()
  }
}

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  let input: TransactionRequest = request();
  let types: HashSet<TraceType> = TraceType::ALL.into();
  let block: BlockId = BlockId::latest();
  let value: TraceCallResult = transport.call(input, types, Some(block))?;
  println!("[trace_call]: {value:#?}");

  let input: [(TransactionRequest, HashSet<TraceType>); 2] = [
    (request(), TraceType::ALL.into()),
    (request(), TraceType::ALL.into()),
  ];
  let block: BlockId = BlockId::latest();
  let value: Vec<TraceCallResult> = transport.call_many(&input, Some(block))?;
  println!("[trace_callMany]: {value:#?}");

  let input: Bytes = "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675".parse().unwrap();
  let types: HashSet<TraceType> = TraceType::ALL.into();
  let value: TraceCallResult = transport.raw_transaction(&input, types)?;
  println!("[trace_rawTransaction]: {value:#?}");

  let input: BlockId = "0x2ed119".parse().unwrap();
  let types: HashSet<TraceType> = TraceType::ALL.into();
  let value: Option<Vec<TraceCallResult>> = transport.replay_block_transactions(input, types)?;
  println!("[trace_replayBlockTransactions]: {value:#?}");

  let input: H256 = "0x983444c7747ffbe25f05a953bc5e4612ea1efa78bd8a7d187d035b3f5c43bc6f".parse().unwrap();
  let types: HashSet<TraceType> = TraceType::ALL.into();
  let value: Option<TraceCallResult> = transport.replay_transaction(input, types)?;
  println!("[trace_replayTransaction]: {value:#?}");

  let input: BlockId = "0x2ed119".parse().unwrap();
  let value: Option<Vec<ParityTrace>> = transport.block(input)?;
  println!("[trace_block]: {value:#?}");

  let input: TraceFilterRequest = filter();
  let value: Option<Vec<ParityTrace>> = transport.filter(input)?;
  println!("[trace_filter]: {value:#?}");

  let input: H256 = "0x17104ac9d3312d8c136b7f44d4b8b47852618065ebfa534bd2d3b5ef218ca1f3".parse().unwrap();
  let index: &[u64] = &[0];
  let value: Option<ParityTrace> = transport.get(input, index)?;
  println!("[trace_get]: {value:#?}");

  let input: H256 = "0x17104ac9d3312d8c136b7f44d4b8b47852618065ebfa534bd2d3b5ef218ca1f3".parse().unwrap();
  let value: Option<Vec<ParityTrace>> = transport.transaction(input)?;
  println!("[trace_transaction]: {value:#?}");

  if cfg!(feature = "node-reth") {
    let input: BlockId = "0x2ed119".parse().unwrap();
    let value: Option<_> = transport.block_opcode_gas(input)?;
    println!("[trace_blockOpcodeGas]: {value:#?}");

    let input: H256 = "0x17104ac9d3312d8c136b7f44d4b8b47852618065ebfa534bd2d3b5ef218ca1f3".parse().unwrap();
    let value: Option<_> = transport.transaction_opcode_gas(input)?;
    println!("[trace_transactionOpcodeGas]: {value:#?}");
  }

  Ok(())
}
