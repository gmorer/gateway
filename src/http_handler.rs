use std::collections::HashMap;
use async_std::io::{ self, Error , ErrorKind};
use async_std::io::{ BufRead, Write };
use async_std::io::prelude::BufReadExt;
use core::marker::Unpin;

#[allow(dead_code)]
enum HttpState {
	Default,
	Header,
	Body,
	Ended
}

#[derive(Debug)]
enum HttpMethod {
	Get,
	Post,
	Put,
	Delete
}

#[derive(Debug)]
enum HttpVersion {
	Http1,
	Http11,
	Http2
}

#[allow(dead_code)]
pub struct HttpHandler<R, W> where R: BufRead + Unpin, W: Write {
	state: HttpState,
	reader: R,
	writer: W,
	headers: HashMap<String, String>,
	body_length: u32
}

fn into_method(met: &str) -> io::Result<HttpMethod> {
	match met {
		"GET" => Ok(HttpMethod::Get),
		"POST" => Ok(HttpMethod::Post),
		"PUT" => Ok(HttpMethod::Put),
		"DELETE" => Ok(HttpMethod::Delete),
		_ => Err(Error::new(ErrorKind::InvalidInput, "Invalid HTTP metode"))
	}
}

fn into_protocol(prot: &str) -> io::Result<HttpVersion> {
	match prot {
		"HTTP/1.0" => Ok(HttpVersion::Http1),
		"HTTP/1.1" => Ok(HttpVersion::Http11),
		"HTTP/2" => Ok(HttpVersion::Http2),
		_ => Err(Error::new(ErrorKind::InvalidInput, "Invalid HTTP version"))
	}
}

fn get_header_args(buf: &str) -> io::Result<(HttpMethod, String, HttpVersion)> {
	let mut split = buf.split_whitespace();
	let method = match split.next() {
		Some(method) => into_method(&method)?,
		None => return Err(Error::new(ErrorKind::InvalidInput, "HTTP first argument missing"))
	};
	let path = match split.next() {
		Some(path) => path.into(),
		None => return Err(Error::new(ErrorKind::InvalidInput, "HTTP second argument missing"))
	};

	let protocol = match split.next() {
		Some(protocol) => into_protocol(&protocol)?,
		None => return Err(Error::new(ErrorKind::InvalidInput, "HTTP third argument missing"))
	};
	Ok((method, path, protocol))
}

impl<R, W> HttpHandler<R, W> where R: BufRead + Unpin, W: Write {
	pub async fn new(mut reader: R, writer: W ) -> io::Result<Self> {
		let mut buf = String::new();
		let size = match reader.read_line(&mut buf).await {
			Ok(0) | Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid first input")),
			Ok(size) => size,
		};
		if buf.ends_with('\n') { buf.pop(); }
		if buf.ends_with('\r') { buf.pop(); }
		let (method, path, protocol) = get_header_args(&buf)?;
		println!("method: [{:?}], path: [{}], protocol: [{:?}]", method, path, protocol);
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