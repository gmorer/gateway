use std::convert::Infallible;
use std::collections::HashMap;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};


// mod utils;
// mod login;
// use login::login;

// mod security;
// mod middlewares;
// use middlewares::{ Jwt };

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

async fn handle_req(modules: modules::Modules, req: Request<Body>) -> Result<Response<Body>> {
	match proto::Request::froom(req) {
		Ok((module, req)) => Ok(modules.call(module, req).await.unwrap_or(proto::Response::new("welllwellwell")).into()),
		Err(res) => Ok(res.into())
	}
}

fn login_sample(_req: proto::Request) -> modules::CallFnRet {
	Box::pin(async move {
		proto::Response::new("SOme response from the future")
	})
}

fn init_login() -> HashMap<String, modules::CallFn> {
	let mut result: HashMap<String, modules::CallFn> = HashMap::new();
	result.insert("test".into(), login_sample);
	result
}

#[tokio::main]
async fn main() {
	let mut modules = modules::Modules::new();
	modules.add_static("auth".to_string(), init_login());
	// We'll bind to 127.0.0.1:3000
	let addr = ADDR.parse().expect("Invalid server address");
	// let db = sled::open(DATABASE_PATH).expect("Cannot open database path"); // put this in an option global
	// println!("example AccessToken: {}", security::create_token("toto".to_string(), security::TokenType::AccessToken));
	// println!("example RefreshToken: {}", security::create_token("toto".to_string(), security::TokenType::RefreshToken));

	
	let new_service = make_service_fn(move |_conn| {
		let modules = modules.clone();
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