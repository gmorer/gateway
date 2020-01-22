/*
	Some utils functions that can be used anywhere
	TODO: create a lib
*/

use actix_web::{ HttpResponse, error };
use actix_web::error::JsonPayloadError::{ Overflow, ContentType, Deserialize, Payload };
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMsg {
	error: String
}

impl ErrorMsg {
	pub fn new<T>(msg: T) -> Self where T: ToString {
		Self { error: msg.to_string() }
	}

	pub fn to_json_string(&self) ->  String {
		format!("{{\"error\":\"{}\"}}", self.error)
	}
}

pub fn handle_json_error(cfg: actix_web::web::JsonConfig) -> actix_web::web::JsonConfig
{

	cfg.limit(4096)
	.error_handler(|err, _req| {
		error::InternalError::from_response("error",
			match err {
				Overflow => HttpResponse::PayloadTooLarge().json(ErrorMsg { error: "Body too large".to_string()}),
				ContentType => HttpResponse::UnsupportedMediaType().json(ErrorMsg { error: "Invalid content Type".to_string()}),
				Deserialize(err) => HttpResponse::BadRequest().json(ErrorMsg { error: err.to_string() }),
				Payload(err) => HttpResponse::BadRequest().json(ErrorMsg { error: err.to_string()})
			}
		).into()
	})
	.content_type(|_mime| {
		true // accept everything
	})
}