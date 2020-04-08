use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use crate::proto::{ Response, Request };

pub enum Error {
	NotFound,
	// Internal(String)
}

// this result is for the use of '?' in the callback functions 
pub type CallFnRet = Pin<Box<dyn Future<Output = Result<Response, Response>> + Send>>;
pub type CallFn = fn(Request) -> CallFnRet;
// pub type CallFn = dyn Fn(Request) -> Response;

// TODO: distant module
/*
struct distantModule {
	readStream
	writeStream
	status
}
*/

#[derive(Clone)]
pub struct Modules {
	static_modules: HashMap<String, HashMap<String, CallFn>>,
	// distant_modules: Vec<distantModule>
}

impl Modules {
	pub fn new() -> Self {
		let static_modules: HashMap<String, HashMap<String, CallFn>> = HashMap::new();
		Modules {
			static_modules,
			// distant_modules: vec!()
		}
	}

	pub fn add_static(&mut self, name: String, method_map: HashMap<String, CallFn>) {
		if self.static_modules.contains_key(&name) { 
			panic!("Cannot insert internal module [{}]: 2 internal module with the same name", name);
		}
		// if self.distant_modules.contains_key(name) {
		// 	panic!("Cannot insert internal module [{}]: A distant modulealready have this name", name);
		// }
		self.static_modules.insert(name, method_map);
	}

	pub async fn call(&self, module: String, req: Request) -> Result<Response, Error> {
		if let Some(module) = self.static_modules.get(&module) {
			match module.get(&req.method) {
				Some(f) => Ok(f(req).await.unwrap_or_else(|e| e)),
				None => {
					// TODO: maybe a better way
					if let Some(f) = module.get("default") {
						Ok(f(req).await.unwrap_or_else(|e| e))
					} else {
						Err(Error::NotFound)
					}
				}
			}
		} else { Err(Error::NotFound) }
	}
}