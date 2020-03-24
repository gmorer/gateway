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

#[derive(Serialize, Deserialize)]
struct User {
	username: String,
	password: String
}

mod answer {
	pub const GOODCREDENTIAL: &str = "Good credentials";
	pub const USERCREATED: &str = "User created";
	pub const USERDELETED: &str = "User deleted";
	pub const INVALIDCREDENTIAL: &str = "Invalid credentials";
	pub const ALREADYEXIST: &str = "Username already exist";
}

/* Return a refresh token and an access token */
// TODO: hash password
// async fn auth(req: ServiceRequest, db: Tree) -> Result<HttpResponse, Error> {
// 	let user: User = match parse_body(req).await {
// 		Ok(user) => user,
// 		Err(e) => return Ok(HttpResponse::BadRequest().json(ErrorMsg { error: e.to_string() }))
// 	};
// 	let password = match db.get(&user.username).unwrap_or(None) {
// 		Some(d) => d,
// 		None => return Ok(HttpResponse::Unauthorized().json(ErrorMsg::new(answer::INVALIDCREDENTIAL)))
// 	};
// 	if password != user.password {
// 		Ok(HttpResponse::Unauthorized().json(ErrorMsg::new(answer::INVALIDCREDENTIAL)))
// 	} else {
// 		Ok(HttpResponse::Ok().body(answer::GOODCREDENTIAL))
// 	}
// }

// /* Return a refresh token and an access token */
// // TODO: hash passowrd
// async fn join(req: ServiceRequest, db: Tree) -> Result<HttpResponse, Error> {
// 	let user: User = match parse_body(req).await {
// 		Ok(user) => user,
// 		Err(e) => return Ok(HttpResponse::BadRequest().json(ErrorMsg { error: e.to_string() }))
// 	};
// 	match db.insert(&user.username, user.password.as_bytes().to_vec()).map_err(ErrorMsg::into_internal_error)? {
// 		Some(_) => HttpResponse::Conflict().json(ErrorMsg::new(answer::ALREADYEXIST)).await,
// 		None => {
// 			db.flush_async().await.map_err(ErrorMsg::into_internal_error)?;
// 			Ok(HttpResponse::Ok().body(answer::USERCREATED))
// 		}
// 	}
// }

/* Delete user */
async fn delete(req: ServiceRequest, db: Tree) -> ServiceResponse {
	// let user: User = match parse_body(req).await {
	// 	Ok(user) => user,
	// 	Err(e) => return Ok(HttpResponse::BadRequest().json(ErrorMsg { error: e.to_string() }))
	// };
	// let password = match db.get(&user.username).unwrap_or(None) {
	// 	Some(d) => d,
	// 	None => return HttpResponse::Unauthorized().finish().await
	// };
	// if password != user.password {
	// 	HttpResponse::Unauthorized().finish().await
	// } else {
	// 	db.remove(&user.username).map_err(ErrorMsg::into_internal_error)?;
	// 	db.flush_async().await.map_err(ErrorMsg::into_internal_error)?;
	// 	Ok(HttpResponse::Ok().body(answer::USERDELETED))
	// }
	println!("Hey");
	req.into_response(HttpResponse::Ok().body(answer::USERDELETED))
}

/* List all users for debug */
// async fn list(db: Tree) -> Result<HttpResponse, Error> {
// 	let result: Vec<String> = db.iter().filter_map(Result::ok)
// 		.map(|(user, _)| String::from(str::from_utf8(&user).unwrap_or("")))
// 		.collect();
// 	Ok(HttpResponse::Ok().json(result))
// }


/* Midleware code part */

struct Inner<'a> {
	validation: Validation,
	deconding_key: DecodingKey<'a>,
	encoding_key: EncodingKey,
	user_db: Tree,
	token_db: Tree
}

impl<'a> Jwt<'a> {
	pub fn new(secret: &'a str, db: Db) -> Self {
		let user_db = db.open_tree("user_db").expect("Cannot create/open the users db");
		let token_db = db.open_tree("token_db").expect("Cannot create/open the users db");
		Jwt(Rc::new( Inner {
			validation: Validation::default(),
			deconding_key: DecodingKey::from_secret((secret).as_bytes()),
			encoding_key: EncodingKey::from_secret((secret).as_bytes()),
			user_db,
			token_db
		}))
	}
}

impl<'a, S, B> Transform<S> for Jwt<'a>
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type InitError = ();
	type Transform = JwtMiddleware<'a, S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ok(JwtMiddleware { service, inner: self.0.clone() })
	}
}
pub struct JwtMiddleware<'a, S> {
	service: S,
	inner: Rc<Inner<'a>>
}


impl<S, B> Service for JwtMiddleware<'_, S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // We only need to hook into the `start` for this middleware.

        let is_logged_in = false; // Change this to see the change in outcome in the browser

        if is_logged_in {
            Either::Left(self.service.call(req))
        } else {
            // Don't forward to /login if we are already on /login
            if req.path() == "/login" {
                Either::Left(self.service.call(req))
            } else {
                Either::Left(delete(req, self.inner.user_db))
            }
        }
    }
}

/*
impl<'a, S, B> Service for JwtMiddleware<'a, S>
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
	S::Future: 'static,
	B: 'static
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	// type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

	fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}

	fn call(&mut self, req: ServiceRequest) -> Self::Future {
		// We only need to hook into the `start` for this middleware.
		// let mut svc = self.service.clone();

		Box::pin(async move {
			let is_logged_in = false; // Change this to see the change in outcome in the browser
			println!("{}", req.path());
			match req.path() {
				"auth" => Ok(req.into_response(auth(req, self.inner.user_db).await?)),
				"join" => Ok(req.into_response(join(req, self.inner.user_db).await?)),
				"delete" => Ok(req.into_response(delete(req, self.inner.user_db).await?)),
				"list" => Ok(req.into_response(list(self.inner.user_db).await?)),
				_ => self.service.call(req).await
			}
			// if is_logged_in {
			// 	Ok(self.service.call(req).await?)
			// } else {
			// 	// Don't forward to /login if we are already on /login
			// 	if req.path() == "/login" {
			// 		Ok(self.service.call(req).await?)
			// 	} else {
			// 		Ok(req.into_response(
			// 			HttpResponse::Found()
			// 				.header(http::header::LOCATION, "/login")
			// 				.finish()
			// 				.into_body(),
			// 		))
			// 	}
			// }
		})
	}
}
*/