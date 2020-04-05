/*
	Some utils functions that can be used anywhere
	TODO: create a lib
*/

// use serde::Serialize;
// use hyper::{ Request, Body };
use crate::proto::{ Request, Response, Code };
// use futures_util::{stream, StreamExt};
use bytes::buf::ext::BufExt;

// #[derive(Serialize)]
// pub struct ErrorMsg {
// 	error: String
// }

// impl ErrorMsg {
// 	pub fn new<T>(msg: T) -> Self where T: ToString {
// 		Self { error: msg.to_string() }
// 	}
	
// 	pub fn to_json_string(&self) ->  String {
// 		json_error(&self.error)
// 	}
// }

pub fn json_error<T>(e: T) -> String where T: std::fmt::Display {
	format!("{{\"error\":\"{}\"}}", e)
}

pub fn into_internal_error<T>(e: T) -> Response where T: std::fmt::Display {
	Response::new(Code::InternalServerError, &json_error(e))
}

// pub fn handle_json_error(cfg: actix_web::web::JsonConfig) -> actix_web::web::JsonConfig
// {

// 	cfg.limit(4096)
// 	.error_handler(|err, _req| {
// 		error::InternalError::from_response("error",
// 			match err {
// 				Overflow => HttpResponse::PayloadTooLarge().json(ErrorMsg { error: "Body too large".to_string()}),
// 				ContentType => HttpResponse::UnsupportedMediaType().json(ErrorMsg { error: "Invalid content Type".to_string()}),
// 				Deserialize(err) => HttpResponse::BadRequest().json(ErrorMsg { error: err.to_string() }),
// 				Payload(err) => HttpResponse::BadRequest().json(ErrorMsg { error: err.to_string()})
// 			}
// 		).into()
// 	})
// 	.content_type(|_mime| {
// 		true // accept everything
// 	})
// }

// #[allow(dead_code)]
// pub struct TestCall<'a> {
// 	pub uri: &'a str,
// 	pub method: Method,
// 	pub body: String,
// 	pub status: StatusCode,
// 	pub response: String
// }

// #[allow(dead_code)]
// pub fn build_test<'a>(uri: &'a str, method: Method, body: &'a str, status: StatusCode, response: String) -> TestCall<'a> {
// 	TestCall { uri, method, body: body.into(), status, response: response.into()}
// }

// // TODO: Option for body, accept empty body
// #[allow(dead_code)]
// pub async fn do_tests<'a, F>(service: F, tests: Vec<TestCall<'a>>) where F: HttpServiceFactory + 'static {
// 	let mut app = test::init_service(
// 		App::new().service(service)
// 	).await;

// 	for (index, test) in tests.iter().enumerate() {
// 		let method = match test.method {
// 			Method::GET => test::TestRequest::get(),
// 			Method::PUT => test::TestRequest::put(),
// 			Method::POST => test::TestRequest::post(),
// 			Method::DELETE => test::TestRequest::delete(),
// 			_ => panic!("Unimplemnted method")
// 		};
// 		// let body = String::from(test.body);
// 		let req = method
// 			.uri(test.uri)
// 			.set_payload(Bytes::from(test.body.clone()))
// 			.header("Content-type", "application/json")
// 			.to_request();
// 		let resp = app.call(req).await.expect(&format!("[{}] Wrong answer", index));
// 		assert_eq!(resp.status(), test.status, "[{}] Response status code invalid -> method: {:?}, uri: {}, body: {}", index, test.method, test.uri, test.body);
// 		let response_body = match resp.response().body().as_ref() {
// 			Some(actix_web::body::Body::Bytes(bytes)) => bytes,
// 			_ => panic!("[{}] Empty body", index),
// 		};
// 		assert_eq!(*response_body, test.response, "[{}] Response body invalid -> method: {:?}, uri: {}, body: {}", index, test.method, test.uri, test.body);
// 		println!("Ok");
// 	};
// 	// tests.forEach
// }

pub async fn parse_body<T>(req: Request) -> Result<T, String> where T: serde::de::DeserializeOwned {
	if let Some(body) = req.body {
		let whole_body = hyper::body::aggregate(body).await.map_err(|e| format!("Invalid body: {}", e))?;
		let data = serde_json::from_reader(whole_body.reader()).map_err(|e| format!("Invalid body: {}", e))?;
		Ok(data)
	} else {
		Err("No body in the request".to_string())
	}
}