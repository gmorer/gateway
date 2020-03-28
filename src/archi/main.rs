use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

mod utils;
mod proto;
// mod login;
// use login::login;

mod security;
// mod middlewares;
// use middlewares::{ Jwt };

const DATABASE_PATH: &str = "db";
const ADDR: &str = "127.0.0.1:8088";

// async fn p404() -> impl Responder {
// 	HttpResponse::NotFound().body("404 not found")
// }

// fn service404() -> Resource {
// 	web::resource("")
// 		.route(web::get().to(p404))
// 		.route(
// 			web::route()
// 				.guard(guard::Not(guard::Get()))
// 				.to(HttpResponse::MethodNotAllowed),
// 		)
// }

async fn shutdown_signal() {
	// Wait for the CTRL+C signal
	tokio::signal::ctrl_c()
		.await
		.expect("failed to install CTRL+C signal handler");
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
	match proto::Request::froom(req) {
		Ok(req) => Ok(Response::new("Hello, World".into())),
		Err(res) => Ok(res.into())
	}
}

#[tokio::main]
async fn main() {
	// We'll bind to 127.0.0.1:3000
	let addr = ADDR.parse().expect("Invalid server address");
	let db = sled::open(DATABASE_PATH).expect("Cannot open database path");
	println!("example AccessToken: {}", security::create_token("toto".to_string(), security::TokenType::AccessToken));
	println!("example RefreshToken: {}", security::create_token("toto".to_string(), security::TokenType::RefreshToken));

	println!("listening on {}.", ADDR);

	// A `Service` is needed for every connection, so this
	// creates one from our `hello_world` function.
	let make_svc = make_service_fn(|_conn| async {
		// service_fn converts our function into a `Service`
		Ok::<_, Infallible>(service_fn(hello_world))
	});

	let server = Server::bind(&addr).serve(make_svc);
	let graceful = server.with_graceful_shutdown(shutdown_signal());
	if let Err(e) = graceful.await {
		eprintln!("server error: {}", e);
	}
}