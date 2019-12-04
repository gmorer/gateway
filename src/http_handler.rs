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

#[derive(Debug, Copy, Clone)]
enum HttpVersion {
	Http1,
	Http11,
	Http2
}

pub enum HttpCode {
	InternalServerError,
	OK
}

fn from_http_code(code: HttpCode) -> String {
	match code {
		HttpCode::InternalServerError => "500 Internal Server Error",
		HttpCode::OK => "200 OK"
	}.into()
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

fn from_protocol(prot: HttpVersion) -> String {
	match prot {
		HttpVersion::Http1 => "HTTP/1.0",
		HttpVersion::Http11 => "HTTP/1.1",
		HttpVersion::Http2 => "HTTP/2.0"
	}.into()
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

#[allow(dead_code)]
pub struct HttpHandler<R, W> where R: BufRead + Unpin, W: Write + Unpin {
	state: HttpState,
	reader: R,
	writer: W,
	headers: HashMap<String, String>,
	body_length: u32,
	protocol: HttpVersion
}

impl<R, W> HttpHandler<R, W> where R: BufRead + Unpin, W: Write + Unpin {
	// For HTTP/2 do not parse the first line during the creartion of the handler, or create a reusable function
	pub async fn new(mut reader: R, writer: W ) -> io::Result<Self> {
		let mut buf = String::new();
		let _size = match reader.read_line(&mut buf).await {
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
			writer,
			protocol
		})
	}

	pub async fn send_response(&mut self, code: HttpCode, res: String) -> io::Result<u64> {
		let res_message = res.as_bytes();
		let res = format!("{} {}\n\rContent-Length: {}\n\r\n\r{}",
			from_protocol(self.protocol),
			from_http_code(code),
			res_message.len(),
			res
		);
		let mut res = res.as_bytes();
		io::copy(&mut res, &mut self.writer).await
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
			i += 1;
		}
		println!("headers: {:?}", self.headers);
		Ok(i)
	}
}