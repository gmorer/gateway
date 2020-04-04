use sled::{ Db, Tree };
use serde::{Deserialize, Serialize};
use std::str;
use crate::proto::{ Response, Request };
use crate::modules::{ CallFnRet, CallFn };
use std::collections::HashMap;

use crate::utils::{ parse_body };

#[derive(Serialize, Deserialize, Debug)]
struct User {
	username: String,
	password: String
}

fn login_sample(req: Request) -> CallFnRet {
	Box::pin(async move {
		let user: User = match parse_body(req).await {
			Ok(user) => user,
			Err(e) => return Response::new(400, "error")
		};
		Response::new(200, &format!("use created: {:?}", user))
	})

	// match db.insert(&user.username, user.password.as_bytes().to_vec()).map_err(ErrorMsg::into_internal_error)? {
	// 	Some(_) => HttpResponse::Conflict().json(ErrorMsg::new(answer::ALREADYEXIST)).await,
	// 	None => {
	// 		db.flush_async().await.map_err(ErrorMsg::into_internal_error)?;
	// 	}
	// }
}

// pub fn login(db: Db, path: &str) -> Scope {
// 	let db = db.open_tree(path).expect("Cannot create/open the users db");
// 	// let data = web::Data::new(db);
// 	web::scope(path)
// 		.data(db)
// 		.route("/list", web::get().to(list))
// 		.app_data(web::Json::<User>::configure(handle_json_error))
// 		.route("/auth", web::post().to(authentification))
// 		.route("/join", web::post().to(join))
// 		.route("/delete", web::delete().to(delete))
// }

pub fn init_login() -> HashMap<String, CallFn> {
	let mut result: HashMap<String, CallFn> = HashMap::new();
	result.insert("join".into(), login_sample);
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