use actix_web::{ Scope, web, HttpResponse, Responder, FromRequest, Error };
use sled::{ Db, Tree };
use serde::{Deserialize, Serialize};
use std::str;

use crate::utils::{ handle_json_error, ErrorMsg };

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