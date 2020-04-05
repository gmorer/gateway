use sled::{ Db, Tree };
use serde::{Deserialize, Serialize};
use std::str;
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use futures::future;

use crate::proto::{ Response, Request, Code };
use crate::modules::{ CallFnRet, CallFn };
use crate::utils::{ parse_body, into_internal_error, json_error };

#[derive(Serialize, Deserialize, Debug)]
struct User {
	username: String,
	password: String
}

#[allow(dead_code)]
mod answer {
	pub const GOODCREDENTIAL: &str = "Good credentials";
	pub const USERCREATED: &str = "User created";
	pub const USERDELETED: &str = "User deleted";
	pub const INVALIDCREDENTIAL: &str = "Invalid credentials";
	pub const ALREADYEXIST: &str = "Username already exist";
}

// users_db(None) to get the db instance and users_db(db) to initalize it when not initilized
fn users_db(db: Option<Db>) -> &'static Tree {
	static INSTANCE: OnceCell<Tree> = OnceCell::new();
	INSTANCE.get_or_init(|| {
		db.expect("cannot initiat the Tree without Db").open_tree("users").expect("Cannot create/open the users db")
	})
}

fn list(_req: Request) -> CallFnRet {
	Box::pin(async move {
		let db = users_db(None);
		let result: Vec<String> = db.iter().filter_map(Result::ok)
			.map(|(user, _)| String::from(str::from_utf8(&user).unwrap_or("")))
			.collect();
		Ok(Response::new(Code::OK, &serde_json::to_string(&result).map_err(into_internal_error)?))
	})
}

fn login_sample(req: Request) -> CallFnRet {
	Box::pin(async move {
		let user: User = match parse_body(req).await {
			Ok(user) => user,
			Err(e) => return Ok(Response::new(Code::BadRequest, &json_error(e)))
		};
		let db = users_db(None);
		match db.insert(&user.username, user.password.as_bytes().to_vec()).map_err(into_internal_error)? {
			Some(_) => Ok(Response::new(Code::Conflict, &json_error(answer::ALREADYEXIST))),
			None => {
				db.flush_async().await.map_err(into_internal_error)?;
				Ok(Response::new(Code::OK, answer::USERCREATED))
			}
		}
	})

}

// pub fn login(db: Db, path: &str) -> Scope {
	// 	// let data = web::Data::new(db);
	// 	web::scope(path)
	// 		.data(db)
	// 		.route("/list", web::get().to(list))
	// 		.app_data(web::Json::<User>::configure(handle_json_error))
	// 		.route("/auth", web::post().to(authentification))
	// 		.route("/join", web::post().to(join))
	// 		.route("/delete", web::delete().to(delete))
	// }

pub fn init_login(db: Db) -> HashMap<String, CallFn> {
	// initialize the users' db
	users_db(Some(db));
	let mut result: HashMap<String, CallFn> = HashMap::new();
	result.insert("join".into(), login_sample);
	result.insert("list".into(), list);
	result
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use actix_web::{ Error };
// 	use actix_web::http::{ Method, StatusCode };
// 	use sled::{ Db, Config };
// 	use crate::utils::{ ErrorMsg, do_tests, build_test };
// 	use lazy_static::lazy_static;

// 	lazy_static!{
// 		static ref DB_TEST: Db = Config::new().temporary(true).open().expect("Cannot create temporary db for tests");
// 	}
	
// 	#[actix_rt::test]
// 	async fn all_tests() -> Result<(), Error> {
// 		let tests = vec![
// 			build_test("/login/join", Method::POST, r##"{"username": "John", "password": "smith"}"##.into(),
// 				StatusCode::OK, answer::USERCREATED.to_string()
// 			),
// 			build_test("/login/join", Method::POST, r##"{"username": "John", "password": "smith"}"##.into(),
// 				StatusCode::CONFLICT, ErrorMsg::new(answer::ALREADYEXIST).to_json_string()
// 			),
// 			build_test("/login/auth", Method::POST, r##"{"username": "John", "password": "Wrongpassword"}"##.into(),
// 				StatusCode::UNAUTHORIZED, ErrorMsg::new(answer::INVALIDCREDENTIAL).to_json_string()
// 			),
// 			build_test("/login/auth", Method::POST, r##"{"username": "John", "password": "smith"}"##.into(),
// 				StatusCode::OK, answer::GOODCREDENTIAL.to_string()
// 			),
// 		];

// 		do_tests(login(DB_TEST.clone(), "/login"), tests).await;
// 		Ok(())
// 	}
// }