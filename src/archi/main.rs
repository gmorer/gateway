use actix_web::{ App, HttpServer };

mod login;
use login::login;

const DATABASE_PATH: &str = "db";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let db = sled::open(DATABASE_PATH).expect("Cannot open database path");
	HttpServer::new(move || {
		App::new()
			.service(login(db.clone(), "/login"))
	})
	.bind("127.0.0.1:8088")?
		.run()
		.await
}
