use ueb3::types::rpc::txpool::Content;
use ueb3::types::rpc::txpool::ContentFrom;
use ueb3::types::rpc::txpool::Inspect;
use ueb3::api::NsTxPool as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::rpc::txpool::Status;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  let value: Content = transport.content()?;
  println!("[txpool_content]: {value:#?}");

  if let Some((input, _)) = value.pending.first_key_value() {
    let value: ContentFrom = transport.content_from(&input)?;
    println!("[txpool_contentFrom]: {value:#?}");
  } else {
    println!("[txpool_contentFrom]: The Transaction Pool is Currently Empty");
  }

  let value: Inspect = transport.inspect()?;
  println!("[txpool_inspect]: {value:#?}");

  let value: Status = transport.status()?;
  println!("[txpool_status]: {value:#?}");

  Ok(())
}
