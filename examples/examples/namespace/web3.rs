use ueb3::api::NsWeb3 as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eth::Bytes;
use ueb3::types::eth::H256;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  let data: Bytes = "0x68656c6c6f20776f726c64".parse()?; // "hello world"
  let hash: H256 = transport.sha3(&data)?;
  let real: H256 = "0x47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad".parse()?;

  println!("[web3_clientVersion]: {}", transport.client_version()?);
  println!("[web3_sha3]:          {}", hash);

  assert_eq!(hash, real);

  Ok(())
}
