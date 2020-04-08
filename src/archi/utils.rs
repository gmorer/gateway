/*
	Some utils functions that can be used anywhere
	TODO: create a lib
*/

use crate::proto::{ Request, Response, Code };
use bytes::buf::ext::BufExt;

pub fn json_error<T>(e: T) -> String where T: std::fmt::Display {
	format!("{{\"error\":\"{}\"}}", e)
}

pub fn into_internal_error<T>(e: T) -> Response where T: std::fmt::Display {
	Response::new(Code::InternalServerError, json_error(e))
}

pub async fn parse_body<T>(req: Request) -> Result<T, String> where T: serde::de::DeserializeOwned {
	if let Some(body) = req.body {
		let whole_body = hyper::body::aggregate(body).await.map_err(|e| format!("Invalid body: {}", e))?;
		let data = serde_json::from_reader(whole_body.reader()).map_err(|e| format!("Invalid body: {}", e))?;
		Ok(data)
	} else {
		Err("No body in the request".to_string())
	}
}