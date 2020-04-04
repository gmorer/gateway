use serde::{ Deserialize, Serialize };
use hyper::{ Body };

#[derive(Debug)]
pub struct Request {
	pub method: String,
	body_len: u64,
	pub body: Option<Body>, /* or stream */
	username: Option<String> /* is the user connected */
	// TODOL role
}

impl Request {
	pub fn froom(req: hyper::Request<Body>) -> Result<(String, Self), Response> {
		// TODO: Handle ascii
		let path = &req.uri().path()[1..];
		let ( module, method ) = match path.find('/') {
			Some( index ) => {
				let module = path[..index].into();
				let method = &path[index + 1..];
				if method.is_empty() {
					return Err(Response::new(500, &"Need a method name"))?;
				}
				// TODO: maybe change char '/' by '_' in method
				( module, method.into() )
			},
			None => {
				if path.is_empty() {
					( "static".into(), "index.html".into() )
				} else {
					return Err(Response::new(500, &"Need a method name"));
				}
			}
		};
		let username = if req.headers().contains_key("Authorization") {
			None // username from token
		} else {
			None
		};
		let ( body_len, body ) = match req.headers().get("Content-Length") {
			Some( value ) => {
				let body_len = value.to_str().map_err(|_| Response::new(500, &"2"))?.parse().map_err(|_| Response::new(500, &"3"))?;
				// let body = hyper::body::aggregate(req).await.map_error(|| Response::new())?.expect(None);
				let body = Some(req.into_body()); // consummate the body
				( body_len, body )
			}
			None => ( 0, None )
		};
		// println!("body_len: {:?}", body_len);
		// println!("body: {:?}", body);
		Ok ((module, Request {
			method,
			body,
			body_len,
			username
		}))
	}
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	code: u16,
	body_len: u64,
	body: String
}

impl Response {
	pub fn new(code: u16, no: &str) -> Self {
		Response {
			body_len: 54,
			code,
			body: no.into()
		}
	}
}

/* impl http code conversion */

impl Into<hyper::Response<Body>> for Response {
	fn into(self) -> hyper::Response<Body> {
		hyper::Response::builder()
			.status(self.code)
			.body(Body::from(self.body)).unwrap_or(
				hyper::Response::new("Internal error when writing response".into())
			)
	}
}