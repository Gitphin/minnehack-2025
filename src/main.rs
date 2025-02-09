#![feature(impl_trait_in_fn_trait_return)]

use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use reqwest::Client;
use tokio::net::TcpListener;

mod food_bank;
mod format;
mod gcloud;
mod helper_funcs;
mod restaurants;

fn resp_builder(
    gcloud_client: Client,
) -> impl Fn(
    Request<hyper::body::Incoming>,
) -> impl Future<Output = Result<Response<Full<Bytes>>, Infallible>> {
    return async move |req: Request<hyper::body::Incoming>| -> Result<Response<Full<Bytes>>, Infallible> {
        match req.uri().path() {
            // example:
            "/example_endpoint" => {
                return Ok(Response::builder().status(200).body(Full::new(Bytes::from("{\"example\": true}"))).unwrap());
            },
            _ => {
                return Ok(Response::builder().status(404).body(Full::new(Bytes::from("404 not found"))).unwrap());
            }
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //food_bank::find_food_shelters(client).await?;
    //restaurants::find_restaurants(client).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await?;

    println!("Bound on all available interfaces at port 3000 (see http://127.0.0.1:3000)");

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let client = gcloud::init().unwrap();

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(resp_builder(client)))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
