/*
| 0    | 8    | 16   | 32   |
| Type | Flag | StatusCode  |
|         Body size         |

{
	Type; u8
	Flag: u8
	StatusCode: u16
	BodySize: u32
}
*/

use async_std::io;
use async_std::net::{ TcpListener, TcpStream };
use async_std::prelude::*;
use async_std::task;
use async_std::io::{ BufReader, BufWriter };
use std::result::Result;

const ADDR: &str = "127.0.0.1";
const PORT: &str = "8080";

mod http_handler;
use http_handler::HttpHandler;

async fn client_handler(stream: TcpStream) {
		let (reader, writer) = &mut (&stream, &stream);
		let reader = BufReader::new(reader);
		let writer = BufWriter::new(writer);
		if let Ok(mut handler) = HttpHandler::new(reader, writer).await {
			println!("ok");
			if let Err(e) = handler.retrieve_headers().await {
				eprintln!("{}", e.to_string()); // send error
			}
		}
		// io::copy(&mut reader, writer).await?;
}

async fn main_server(addr: &str, port: &str) -> io::Result<()> {
	let listener = TcpListener::bind(format!("{}:{}", addr, port)).await?;
	let mut incoming = listener.incoming();

	println!("Listening on: {}:{}", addr, port);
	while let Some(stream) = incoming.next().await { client_handler(stream?).await; }
	Ok(())
}

fn main() -> Result<(), std::io::Error> {
	task::block_on(async {
		main_server(ADDR, PORT).await
	})
}