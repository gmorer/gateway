use std::collections::HashMap;
use async_std::io::{ self, Error , ErrorKind};
use async_std::io::{ BufRead, Write };
use async_std::io::prelude::BufReadExt;
use core::marker::Unpin;

enum HttpState {
	Default,
	Header,
	Body,
	Ended
}

pub struct HttpHandler<R, W> where R: BufRead + Unpin, W: Write {
	state: HttpState,
	reader: R,
	writer: W,
	headers: HashMap<String, String>,
	body_length: u32
}

impl<R, W> HttpHandler<R, W> where R: BufRead + Unpin, W: Write {
	pub async fn new(mut reader: R, writer: W ) -> io::Result<Self> {
		let mut buf = String::new();
		let size = match reader.read_line(&mut buf).await {
			Ok(0) | Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid first input")),
			Ok(size) => size,
		};
		if buf.ends_with('\n') { buf.pop(); }
		println!("First header line: [{}]", buf);
		return Ok( HttpHandler {
			state: HttpState::Default,
			headers: HashMap::new(),
			body_length: 0,
			reader,
			writer
		})
	}

	pub async fn retrieve_headers(&mut self) -> io::Result<usize> {
		let mut i = 0;
		loop {
			let mut buf = String::new();
			match self.reader.read_line(&mut buf).await {
				Ok(0) => break,
				Ok(_) => (),
				Err(e) => return Err(e)
			};
			if buf.ends_with('\n') { buf.pop(); }
			if buf.ends_with('\r') { buf.pop(); }
			if buf.is_empty() { break }
			let pos = match buf.find(':') {
				None => return Err(Error::new(ErrorKind::InvalidInput, "Invalid header")),
				Some(pos) => pos
			};
			buf.remove(pos);
			let (key, value) = buf.split_at(pos);
			self.headers.insert(key.into(), value.into());
			// println!("---{}ooo", buf);
			i += 1;
		}
		println!("headers: {:?}", self.headers);
		Ok(i)
	}
}