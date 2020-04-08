use std::collections::HashMap;
use tokio::fs::File;
use std::path::Path;
use tokio::io::AsyncReadExt;
use hyper::Body;
use crate::modules::{ CallFnRet, CallFn };
use crate::proto::{ Response, Request, Code };
use crate::utils::{ into_internal_error };

const STATIC_PATH: &str = "C:\\Users\\flust\\Documents\\projects\\gateway\\static";

// TODO: change how we proceed : create a strean from a wrap the stream to the response
// Or place each file of STATIC_PATH in a Hashmap with the path and the content of the file
fn serve(req: Request) -> CallFnRet {
	Box::pin(async move {
		let filename = Path::new(STATIC_PATH).join(req.method);
		let mut file = File::open(filename).await.map_err(|_| Response::new(Code::NotFound, "Error 404"))?;
		let mut buf = Vec::new();
		file.read_to_end(&mut buf).await.map_err(into_internal_error)?;
		Ok(Response::new(Code::OK, Body::from(buf)))
	})
}

pub fn init_static() -> HashMap<String, CallFn> {
	let mut result: HashMap<String, CallFn> = HashMap::new();
	result.insert("default".into(), serve);
	result
}
