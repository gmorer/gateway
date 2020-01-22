use actix_web::{ Scope, web, HttpResponse, Responder };
use sled::{ Db, Tree };
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
struct User {
	username: String,
	password: String
}

async fn authentification(db: web::Data<Tree>, user: web::Json<User>) -> impl Responder {
	println!("i'm in auht");
	let password = match db.get(&user.username).unwrap_or(None) {
		Some(d) => d,
		None => return HttpResponse::Unauthorized().finish()
	};
	if password != user.password {
		HttpResponse::Unauthorized().finish()
	} else {
		HttpResponse::Ok().body("Good credentials")
	}
}

async fn join(db: web::Data<Tree>, user: web::Json<User>) -> impl Responder {
	println!("i'm in join");
	match db.insert(&user.username, user.password.as_bytes().to_vec()).unwrap() {
		Some(_) => HttpResponse::Conflict().finish(),
		None => {
			db.flush_async().await;
			HttpResponse::Ok().body("User created")
		}
	}
}

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
		HttpResponse::Ok().body("User deleted")
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
		.data(web::JsonConfig::default().limit(4096))
		.route("/auth", web::post().to(authentification))
		.route("/join", web::post().to(join))
		.route("/delete", web::delete().to(delete))
}