use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

mod utils;
mod static_modules;
use static_modules::{ login, staticfs };

mod security;

mod proto;
mod modules;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

const DATABASE_PATH: &str = "db";
const ADDR: &str = "127.0.0.1:8088";

async fn shutdown_signal() {
	// Wait for the CTRL+C signal
	tokio::signal::ctrl_c()
		.await
		.expect("failed to install CTRL+C signal handler");
}

// TODO handle content encoding based on the Accept header comming
async fn handle_req(modules: modules::Modules, req: Request<Body>) -> Result<Response<Body>> {
	// if request is a GET and is asking for html, send static/index.html (Care fore the infinit loop)
	match proto::Request::froom(req) {
		Ok((module, req)) => Ok(modules.call(module, req).await.unwrap_or(proto::Response::new(proto::Code::NotFound, "404 not found")).into()),
		Err(res) => Ok(res.into())
	}
}

#[tokio::main]
async fn main() {
	let mut modules = modules::Modules::new();
	let db = sled::open(DATABASE_PATH).expect("Cannot open database path"); // put this in an option global
	modules.add_static("auth".to_string(), login::init_login(db));
	modules.add_static("static".to_string(), staticfs::init_static());
	// We'll bind to 127.0.0.1:3000
	let addr = ADDR.parse().expect("Invalid server address");
	println!("example AccessToken: {}", security::create_token("toto".to_string(), security::TokenType::AccessToken));
	println!("example RefreshToken: {}", security::create_token("toto".to_string(), security::TokenType::RefreshToken));

	
	let new_service = make_service_fn(move |_conn| {
		let modules = modules.clone();
		// TODO: variable with the client loged or not ( stronger than refresh token )
		async move {
			Ok::<_, GenericError>(service_fn(move |req| {
				handle_req(modules.clone(), req)
			}))
		}
	});
	
	println!("listening on {}.", ADDR);
	let server = Server::bind(&addr).serve(new_service);
	let graceful = server.with_graceful_shutdown(shutdown_signal());
	if let Err(e) = graceful.await {
		eprintln!("server error: {}", e);
	}
}