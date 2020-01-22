use actix_web::{ Scope, web, HttpResponse, Responder, FromRequest };
use sled::{ Db, Tree };
use serde::{Deserialize, Serialize};
use std::str;

use crate::utils::{ handle_json_error, ErrorMsg };

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

async fn authentification(db: web::Data<Tree>, user: web::Json<User>) -> impl Responder {
	println!("i'm in auht");
	let password = match db.get(&user.username).unwrap_or(None) {
		Some(d) => d,
		None => return HttpResponse::Unauthorized().json(ErrorMsg::new(answer::INVALIDCREDENTIAL))
	};
	if password != user.password {
		HttpResponse::Unauthorized().json(ErrorMsg::new(answer::INVALIDCREDENTIAL))
	} else {
		HttpResponse::Ok().body(answer::GOODCREDENTIAL)
	}
}

async fn join(db: web::Data<Tree>, user: web::Json<User>) -> impl Responder {
	println!("i'm in join");
	match db.insert(&user.username, user.password.as_bytes().to_vec()).unwrap() {
		Some(_) => HttpResponse::Conflict().json(ErrorMsg::new(answer::ALREADYEXIST)),
		None => {
			db.flush_async().await;
			HttpResponse::Ok().body(answer::USERCREATED)
		}
	}
}

// rework this one get username in body or in params
async fn delete(db: web::Data<Tree>, user: web::Json<User>) -> impl Responder {
	let password = match db.get(&user.username).unwrap_or(None) {
		Some(d) => d,
		None => return HttpResponse::Unauthorized().finish()
	};
	if password != user.password {
		HttpResponse::Unauthorized().finish()
	} else {
		db.remove(&user.username);
		db.flush_async().await;
		HttpResponse::Ok().body(answer::USERDELETED)
	}
}

async fn list(db: web::Data<Tree>) -> impl Responder {
	let result: Vec<String> = db.iter().filter_map(Result::ok)
		.map(|(user, _)| String::from(str::from_utf8(&user).unwrap_or("")))
		.collect();
	HttpResponse::Ok().json(result)
}

pub fn login(db: Db, path: &str) -> Scope {
	let db = db.open_tree(path).expect("Cannot create/open the users db");
	// let data = web::Data::new(db);
	web::scope(path)
		.data(db)
		.route("/list", web::get().to(list))
		.app_data(web::Json::<User>::configure(handle_json_error))
		.route("/auth", web::post().to(authentification))
		.route("/join", web::post().to(join))
		.route("/delete", web::delete().to(delete))
}


#[cfg(test)]
mod tests {
	use super::*;
    use actix_web::dev::Service;
	use actix_web::{http, test, App, Error};
	use sled::{ Db, Config };

	use lazy_static::lazy_static;

	lazy_static!{
		static ref DB_TEST: Db = Config::new().temporary(true).open().expect("Cannot create temporary db for tests");
	}
	
	#[actix_rt::test]
	async fn create_user() -> Result<(), Error> {
		let mut app = test::init_service(
			App::new().service(login(DB_TEST.clone(), "/login"))
		).await;
		// Normall register
		let req = test::TestRequest::post()
			.uri("/login/join")
			.set_payload(r##"{"username": "John", "password": "smith"}"##)
			.header("Content-type", "application/json")
			.to_request();
		let resp = app.call(req).await.expect("Wrong answer");
		assert_eq!(resp.status(), http::StatusCode::OK);
		let response_body = match resp.response().body().as_ref() {
			Some(actix_web::body::Body::Bytes(bytes)) => bytes,
			_ => panic!("Create user response error"),
		};
		assert_eq!(response_body, answer::USERCREATED);
		// Test user with the same name so should be an error
		let req = test::TestRequest::post()
			.uri("/login/join")
			.set_payload(r##"{"username": "John", "password": "smith"}"##)
			.header("Content-type", "application/json")
			.to_request();
		let resp = app.call(req).await.expect("Wrong answer");
		assert_eq!(resp.status(), http::StatusCode::CONFLICT);
		let response_body = match resp.response().body().as_ref() {
			Some(actix_web::body::Body::Bytes(bytes)) => bytes,
			_ => panic!("Create user response error"),
		};
		assert_eq!(response_body, &ErrorMsg::new(answer::ALREADYEXIST).to_json_string());
		Ok(())
	}
}