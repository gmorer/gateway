use std::collections::HashMap;
use async_std::net::TcpStream;
use async_std::io::{ BufReader, BufWriter };
use async_std::io::{ self, Error , ErrorKind};

enum HttpState {
	Default,
	Header,
	Body,
	Ended
}

pub struct HttpHandler<'a> {
	state: HttpState,
	reader: &'a mut BufReader<&'a mut &'a TcpStream>,
	writer: &'a mut BufWriter<&'a mut &'a TcpStream>,
	headers: HashMap<String, String>,
	body_length: u32
}

impl<'a> HttpHandler<'a> {
    pub async fn new(reader: &mut BufReader<&'a mut &'a TcpStream>, writer: &mut BufWriter<&'a mut &'a TcpStream> ) -> io::Result<Self> {
        let mut buf = String::new();
        // let size = match reader.read_line(&mut buf).await {
        //     Ok(0) | Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid first input")),
        //     Ok(size) => size,
        // };
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