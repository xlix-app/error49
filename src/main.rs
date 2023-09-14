mod utils;
mod binds;

use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use anyhow::{Result, anyhow};
use hyper::{Request, Response};
use hyper::body::Bytes;
use hyper::header::CONTENT_TYPE;
use hyper::http::HeaderValue;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use http_body_util::Full;
use hyper_util::rt::TokioIo;
use binds::*;
use utils::*;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let listener = TcpListener::bind(
        get_bind_address(&args).unwrap()
    ).await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let tokio_io = TokioIo::new(stream);

            // No need to log errors.
            let _ = http1::Builder::new()
                .serve_connection(tokio_io, service_fn(hub))
                .await;
        });
    }
}

/// Handles each request.
///
/// Currently all requests will result in the same page,
/// so there is no need to implement additional functionality
async fn hub(_: Request<hyper::body::Incoming>) -> std::result::Result<Response<Full<Bytes>>, Infallible> {
    let response = Response::builder()
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .body::<Full<Bytes>>(Bytes::from(HTML_PAGE).into())
        .unwrap();  // Will never produce an error.

    Ok(response)
}
