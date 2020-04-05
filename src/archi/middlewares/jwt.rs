use std::task::{Context, Poll};
use std::rc::Rc;
use std::pin::Pin;
use std::str;

use sled::{ Db, Tree };
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse, web, Responder, HttpMessage };
use futures::future::{ok, Either, Ready};
use futures::stream::StreamExt;
use jsonwebtoken::{Validation, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use bytes::BytesMut;

use crate::utils::{ /*parse_body,*/ ErrorMsg };

pub struct Jwt<'a>(Rc<Inner<'a>>);

/* Return a refresh token and an access token */
// TODO: hash passowrd
async fn join(req: ServiceRequest, db: Tree) -> Result<HttpResponse, Error> {
	let user: User = match parse_body(req).await {
		Ok(user) => user,
		Err(e) => return Ok(HttpResponse::BadRequest().json(ErrorMsg { error: e.to_string() }))
	};
	match db.insert(&user.username, user.password.as_bytes().to_vec()).map_err(into_internal_error)? {
		Some(_) => HttpResponse::Conflict().json(ErrorMsg::new(answer::ALREADYEXIST)).await,
		None => {
			db.flush_async().await.map_err(into_internal_error)?;
			Ok(HttpResponse::Ok().body(answer::USERCREATED))
		}
	}
}


/* List all users for debug */


/* Midleware code part */