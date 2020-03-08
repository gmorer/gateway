use actix_web::{ Scope, web, HttpResponse, Responder, FromRequest, Error };
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

async fn join(db: web::Data<Tree>, user: web::Json<User>) -> Result<HttpResponse, Error> {
	match db.insert(&user.username, user.password.as_bytes().to_vec()).map_err(ErrorMsg::into_internal_error)? {
		Some(_) => HttpResponse::Conflict().json(ErrorMsg::new(answer::ALREADYEXIST)).await,
		None => {
			db.flush_async().await.map_err(ErrorMsg::into_internal_error)?;
			Ok(HttpResponse::Ok().body(answer::USERCREATED))
		}
	}
}

// rework this one get username in body or in params
async fn delete(db: web::Data<Tree>, user: web::Json<User>) -> Result<HttpResponse, Error> {
	let password = match db.get(&user.username).unwrap_or(None) {
		Some(d) => d,
		None => return HttpResponse::Unauthorized().finish().await
	};
	if password != user.password {
		HttpResponse::Unauthorized().finish().await
	} else {
		db.remove(&user.username).map_err(ErrorMsg::into_internal_error)?; 
		db.flush_async().await.map_err(ErrorMsg::into_internal_error)?;
		Ok(HttpResponse::Ok().body(answer::USERDELETED))
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
	use actix_web::{ Error };
	use actix_web::http::{ Method, StatusCode };
	use sled::{ Db, Config };
	use crate::utils::{ ErrorMsg, do_tests, build_test };
	use lazy_static::lazy_static;

	lazy_static!{
		static ref DB_TEST: Db = Config::new().temporary(true).open().expect("Cannot create temporary db for tests");
	}
	
	#[actix_rt::test]
	async fn all_tests() -> Result<(), Error> {
		let tests = vec![
			build_test("/login/join", Method::POST, r##"{"username": "John", "password": "smith"}"##.into(),
				StatusCode::OK, answer::USERCREATED.to_string()
			),
			build_test("/login/join", Method::POST, r##"{"username": "John", "password": "smith"}"##.into(),
				StatusCode::CONFLICT, ErrorMsg::new(answer::ALREADYEXIST).to_json_string()
			),
			build_test("/login/auth", Method::POST, r##"{"username": "John", "password": "Wrongpassword"}"##.into(),
				StatusCode::UNAUTHORIZED, ErrorMsg::new(answer::INVALIDCREDENTIAL).to_json_string()
			),
			build_test("/login/auth", Method::POST, r##"{"username": "John", "password": "smith"}"##.into(),
				StatusCode::OK, answer::GOODCREDENTIAL.to_string()
			),
		];

		do_tests(login(DB_TEST.clone(), "/login"), tests).await;
		Ok(())
	}
}