mod binds;
mod utils;

use binds::*;
use utils::*;

use std::convert::Infallible;
use std::env;
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use http_body_util::Full;
use hyper::{
    body::Bytes,
    header::CONTENT_TYPE,
    http::HeaderValue,
    server::conn::http1,
    service::service_fn,
    Request,
    Response,
};


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let listener = TcpListener::bind(get_bind_address(&args).unwrap())
        .await
        .unwrap();

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
async fn hub(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let response = Response::builder()
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .body::<Full<Bytes>>(Bytes::from(HTML_PAGE).into())
        .unwrap(); // Will never produce an error.

    Ok(response)
}
