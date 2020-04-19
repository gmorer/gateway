use sled::{ Db, Tree };
use serde::{Deserialize, Serialize};
use std::str;
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use argon2::{self, Config};
use rand::{thread_rng, Rng};

use crate::proto::{ Response, Request, Code };
use crate::modules::{ CallFnRet, CallFn };
use crate::utils::{ parse_body, into_internal_error, json_error };

// temporary, in wait of the mentoring implementation
const MENTORING_KEY: &str = "saitama";

#[derive(Serialize, Deserialize, Debug)]
struct User {
	username: String,
	password: String,
	mentoring: Option<String>
}

mod answer {
	pub const GOODCREDENTIAL: &str = "Good credentials";
	pub const USERCREATED: &str = "User created";
	pub const USERDELETED: &str = "User deleted";
	pub const INVALIDCREDENTIAL: &str = "Invalid credentials";
	pub const ALREADYEXIST: &str = "Username already exist";
	pub const MENTORING_KEY_NOT_PRESENT: &str = "Mentoring key not present";
	pub const INVALID_MENTORING_KEY: &str = "Invalid mentoring key";
}

// users_db(None) to get the db instance and users_db(db) to initalize it when not initilized
fn users_db(db: Option<Db>) -> &'static Tree {
	static INSTANCE: OnceCell<Tree> = OnceCell::new();
	INSTANCE.get_or_init(|| {
		db.expect("cannot initiat the Tree without Db").open_tree("users").expect("Cannot create/open the users db")
	})
}

/* Return a refresh token and an access token */
// TODO: hash password
fn auth(req: Request) -> CallFnRet {
	Box::pin(async move {
		let user: User = match parse_body(req).await {
			Ok(user) => user,
			Err(e) => return Ok(Response::new(Code::BadRequest, json_error(e)))
		};
		let db = users_db(None);
		let hash = match db.get(&user.username).unwrap_or(None) {
			Some(d) => d,
			None => return Ok(Response::new(Code::Unauthorized, json_error(answer::INVALIDCREDENTIAL)))
		};
		match argon2::verify_encoded(str::from_utf8(hash.as_ref()).map_err(into_internal_error)?, user.password.as_bytes()).map_err(into_internal_error)? {
			true => Ok(Response::new(Code::OK, answer::GOODCREDENTIAL.to_string())),
			false => Ok(Response::new(Code::Unauthorized, json_error(answer::INVALIDCREDENTIAL)))
		}
	})
}

fn join(req: Request) -> CallFnRet {
	Box::pin(async move {
		let user: User = match parse_body(req).await {
			Ok(user) => user,
			Err(e) => return Ok(Response::new(Code::BadRequest, json_error(e)))
		};
		if let Some(mentoring) = user.mentoring {
			if mentoring != MENTORING_KEY {
				return Ok(Response::new(Code::BadRequest, json_error(answer::INVALID_MENTORING_KEY)));
			}
		} else {
			return Ok(Response::new(Code::BadRequest, json_error(answer::MENTORING_KEY_NOT_PRESENT)));
		}
		let db = users_db(None);
		let mut salt = [0u8; 10];
		thread_rng().try_fill(&mut salt[..]).map_err(into_internal_error)?;
		let hash = argon2::hash_encoded(user.password.as_bytes(), &salt, &Config::default()).map_err(into_internal_error)?;
		match db.insert(user.username.as_bytes(), hash.as_bytes()).map_err(into_internal_error)? {
			Some(_) => Ok(Response::new(Code::Conflict, json_error(answer::ALREADYEXIST))),
			None => {
				db.flush_async().await.map_err(into_internal_error)?;
				Ok(Response::new(Code::OK, answer::USERCREATED.to_string()))
			}
		}
	})

}

/* Delete user */
fn delete(req: Request) -> CallFnRet {
	Box::pin(async move {
		let user: User = match parse_body(req).await {
			Ok(user) => user,
			Err(e) => return Ok(Response::new(Code::BadRequest, json_error(e)))
		};
		let db = users_db(None);
		let password = match db.get(&user.username).unwrap_or(None) {
			Some(d) => d,
			None => return Ok(Response::new(Code::Unauthorized, answer::INVALIDCREDENTIAL.to_string()))
		};
		if password != user.password {
			Ok(Response::new(Code::Unauthorized, answer::INVALIDCREDENTIAL.to_string()))
		} else {
			db.remove(&user.username).map_err(into_internal_error)?;
			db.flush_async().await.map_err(into_internal_error)?;
			Ok(Response::new(Code::OK, answer::USERDELETED))
		}
	})
}

fn list(_req: Request) -> CallFnRet {
	Box::pin(async move {
		let db = users_db(None);
		let result: Vec<String> = db.iter().filter_map(Result::ok)
			.map(|(user, _)| String::from(str::from_utf8(&user).unwrap_or("")))
			.collect();
		Ok(Response::new(Code::OK, serde_json::to_string(&result).map_err(into_internal_error)?))
	})
}

pub fn init_login(db: Db) -> HashMap<String, CallFn> {
	let mut result: HashMap<String, CallFn> = HashMap::new();
	// initialize the users' db
	users_db(Some(db));
	result.insert("join".into(), join);
	result.insert("list".into(), list);
	result.insert("auth".into(), auth);
	result.insert("delete".into(), delete);
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