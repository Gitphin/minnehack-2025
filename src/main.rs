use std::convert::Infallible;
use std::net::SocketAddr;
use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::Mutex;
use actix_web::HttpServer;
use actix_web::App;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use crate::food_bank::find_food_shelters_route;

mod format;
mod food_bank;
mod restaurants;
mod helper_funcs;
mod gcloud;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Is this working?"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = gcloud::init()?;
    
    let client_data = Data::new(Arc::new(Mutex::new(client.clone())));

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(client_data.clone()) // Share client with Actix
            .service(find_food_shelters_route)
    })
    .bind("0.0.0.0:3001")? // Use a different port for Actix
    .run();

    let hyper_server = async {
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        let listener = TcpListener::bind(addr).await?;
    
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
    
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(hello))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }

        #[allow(unreachable_code)]
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    };

    tokio::select! {
        _ = http_server => (),
        _ = hyper_server => (),
    }

    Ok(())
}

