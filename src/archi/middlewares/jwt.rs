use std::task::{Context, Poll};
use std::rc::Rc;

use sled::{ Db, Tree };
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use jsonwebtoken::{Validation, DecodingKey, EncodingKey};

pub struct Jwt<'a>(Rc<Inner<'a>>);

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

impl<'a, S, B> Service for JwtMiddleware<'a, S>
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
		println!("{}", req.path());
		// match req.path() {
		// 	"list" =>
		// 	"auth" =>
		// 	"join" =>
		// 	"delete" =>
		// }
		if is_logged_in {
			Either::Left(self.service.call(req))
		} else {
			// Don't forward to /login if we are already on /login
			if req.path() == "/login" {
				Either::Left(self.service.call(req))
			} else {
				Either::Right(ok(req.into_response(
					HttpResponse::Found()
						.header(http::header::LOCATION, "/login")
						.finish()
						.into_body(),
				)))
			}
		}
	}
}
