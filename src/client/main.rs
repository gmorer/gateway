use std::net::SocketAddr;
// use capnp_rpc::{RpcSystem, twoparty::VatNetwork, rpc_twoparty_capnp::Side};
// use schemas::test_capnp::{ theinterf, ma_struct };
use services::contact::ContactsClient;
use tarpc::rpc::client::{ Config };
use tarpc::{ context, serde_transport };
use tokio_serde::formats::Json;
use futures::join;
use std::time::Instant;

const ADDR: &str = "127.0.0.1:8080";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
	let server_addr = ADDR.parse::<SocketAddr>()?;
	let transport = serde_transport::tcp::connect(server_addr, Json::default()).await?;
	let mut client = ContactsClient::new(Config::default(), transport).spawn()?;
	let mut client2 = client.clone();
	let now = Instant::now();
	let ( res1, res2 ) = join!(
		client.test(context::current(), 3),
		client2.test(context::current(), 2)
	);
	println!("{}", now.elapsed().as_secs());
	println!("res1: {:?}", res1);
	println!("res1: {:?}", res2);
	Ok(())
}