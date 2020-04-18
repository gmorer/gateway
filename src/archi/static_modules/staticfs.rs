use std::collections::HashMap;
use std::path::Path;
use std::{ io, cmp };
use std::pin::Pin;
use std::task::Poll;
use std::io::SeekFrom;
use std::fs::Metadata;
use bytes::{ Bytes, BytesMut };
use futures::{ stream, future, ready, Stream, FutureExt, StreamExt };
use futures::future::Either;
use tokio::io::AsyncRead;
use tokio::fs::File;
use hyper::Body;
use crate::modules::{ CallFnRet, CallFn };
use crate::proto::{ Response, Request, Code };
use crate::utils::{ into_internal_error };

const STATIC_PATH: &str = "C:\\Users\\flust\\Documents\\projects\\gateway\\static";

/* big thx to the arp crate for the file to wrap_stream from a file for the hyper body */

fn reserve_at_least(buf: &mut BytesMut, cap: usize) {
	if buf.capacity() - buf.len() < cap {
		buf.reserve(cap);
	}
}

fn file_stream(mut file: File, buf_size: usize, (start, end): (u64, u64)) -> impl Stream<Item = Result<Bytes, io::Error>> + Send {
	let seek = async move {
		if start != 0 {
			file.seek(SeekFrom::Start(start)).await?;
		}
		Ok(file)
	};
	seek.into_stream()
		.map(move |result| {
			let mut buf = BytesMut::new();
			let mut len = end - start;
			let mut f = match result {
				Ok(f) => f,
				Err(f) => return Either::Left(stream::once(future::err(f))),
			};
			// create a new stream
			Either::Right(stream::poll_fn(move |cx| {
				if len == 0 {
					return Poll::Ready(None);
				}
				reserve_at_least(&mut buf, buf_size);
				// write(fd, buff, buff.len)
				let n = match ready!(Pin::new(&mut f).poll_read_buf(cx, &mut buf)) {
					Ok(n) => n as u64,
					Err(err) => {
						// log::debug!("file read error: {}", err);
						return Poll::Ready(Some(Err(err)));
					}
				};
				// wasnt expecting this result
				if n == 0 {
					// log::debug!("file read found EOF before expected length");
					return Poll::Ready(None);
				}

				let mut chunk = buf.split().freeze();
				if n > len {
					chunk = chunk.split_to(len as usize);
					len = 0;
				} else {
					len -= n;
				}
				Poll::Ready(Some(Ok(chunk)))
			}))
		})
		.flatten()
}


const DEFAULT_READ_BUF_SIZE: usize = 8_192;

#[cfg(unix)]
fn get_block_size(metadata: &Metadata) -> usize {
	use std::os::unix::fs::MetadataExt;
	//TODO: blksize() returns u64, should handle bad cast...
	//(really, a block size bigger than 4gb?)

	// Use device blocksize unless it's really small.
	cmp::max(metadata.blksize() as usize, DEFAULT_READ_BUF_SIZE)
}

#[cfg(not(unix))]
fn get_block_size(_metadata: &Metadata) -> usize {
	DEFAULT_READ_BUF_SIZE
}

fn optimal_buf_size(metadata: &Metadata) -> usize {
	let block_size = get_block_size(metadata);

	// If file length is smaller than block size, don't waste space
	// reserving a bigger-than-needed buffer.
	cmp::min(block_size as u64, metadata.len()) as usize
}

// TODO: change how we proceed : create a strean from a wrap the stream to the response
// Or place each file of STATIC_PATH in a Hashmap with the path and the content of the file
fn serve(req: Request) -> CallFnRet {
	Box::pin(async move {
		let filename = Path::new(STATIC_PATH).join(req.method);
		let file = File::open(filename).await.map_err(|_| Response::new(Code::NotFound, "Error 404"))?;
		let meta = file.metadata().await.map_err(into_internal_error)?;
		let buf_size = optimal_buf_size(&meta);
		let (start, end) = (0, meta.len());	
		let stream = file_stream(file, buf_size, (start, end));
		let body = Body::wrap_stream(stream);
		Ok(Response::new(Code::OK, body))
	})
}

pub fn init_static() -> HashMap<String, CallFn> {
	let mut result: HashMap<String, CallFn> = HashMap::new();
	result.insert("default".into(), serve);
	result
}
