use actix_web::{ App, HttpServer, HttpResponse, Responder, web, guard, Resource };

mod utils;

mod login;
use login::login;

const DATABASE_PATH: &str = "db";
const ADDR: &str = "127.0.0.1:8088";

async fn p404() -> impl Responder {
	HttpResponse::NotFound().body("404 not found")
}

fn service404() -> Resource {
	web::resource("")
		.route(web::get().to(p404))
		.route(
			web::route()
				.guard(guard::Not(guard::Get()))
				.to(HttpResponse::MethodNotAllowed),
		)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	let db = sled::open(DATABASE_PATH).expect("Cannot open database path");
	println!("listening on {}.", ADDR);
	HttpServer::new(move || {
		App::new()
			.service(login(db.clone(), "/login"))
			.default_service(service404())
	})
	.bind(ADDR)?
		.run()
		.await
}
