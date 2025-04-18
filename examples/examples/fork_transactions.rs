use ueb3::api::Provider as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eth::H256;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn inspect(name: &'static str, transport: &RpcClient, hash: H256) -> Result<()> {
  println!("[{name}] {}", "-".repeat(80 - 2 - name.len()));

  let Some(transaction) = transport.transaction(hash.into())? else {
    panic!("Invalid Fork Transaction: {hash}");
  };

  let Some(receipt) = transport.transaction_receipt(hash)? else {
    panic!("Invalid Fork Transaction: {hash}");
  };

  println!("{:#?}", transaction);
  println!("{:#?}", receipt);

  assert_eq!(transaction.header.hash, receipt.transaction_hash);
  assert_eq!(transaction.header.index, Some(receipt.transaction_index));
  assert_eq!(transaction.header.block_hash, Some(receipt.block_hash));
  assert_eq!(transaction.header.block_number, Some(receipt.block_number));

  println!("[{name}] {}", "-".repeat(80 - 2 - name.len()));

  Ok(())
}

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  // First Legacy Transaction
  let hash_legacy: H256 = "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060".parse()?;

  // First EIP-2930 Transaction
  let hash_eip_2930: H256 = "0x851bad0415758075a1eb86776749c829b866d43179c57c3e4a4b9359a0358231".parse()?;

  // First EIP-1559 Transaction
  let hash_eip_1559: H256 = "0xed382cb554ad10e94921d263a56c670669d6c380bbdacdbf96fed625b7132a1d".parse()?;

  // First EIP-4844 Transaction
  let hash_eip_4844: H256 = "0x9c1fbda4f649ac806ab0faefbe94e1a60282eb374ead6aa01bac042f52b28a8c".parse()?;

  // First EIP-7702 Transaction
  let hash_eip_7702: H256 = "0xafc2e5a0c77aacd5ac75717fa95005e506534909f4a3db008bfe7a8e32d76569".parse()?;

  inspect("Legacy", &transport, hash_legacy)?;
  inspect("EIP-2930", &transport, hash_eip_2930)?;
  inspect("EIP-1559", &transport, hash_eip_1559)?;
  inspect("EIP-4844", &transport, hash_eip_4844)?;
  inspect("EIP-7702", &transport, hash_eip_7702)?;

  Ok(())
}
