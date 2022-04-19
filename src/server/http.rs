use crate::config::KustodioConfiguration;
use http::StatusCode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rust_embed::RustEmbed;
use std::borrow::Borrow;
use std::{convert::Infallible, net::SocketAddr, path::Path, sync::Arc};

const INDEX: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "ui/dist"]
struct UIAssets;

async fn handle(req: Request<Body>, config: Arc<String>) -> Result<Response<Body>, Infallible> {
    let builder = Response::builder();
    let req_path = req.uri().path();
    info!("Serving {}", req_path);
    let path = Path::new(match req_path {
        "/" => "/index.html",
        "/metrics" => todo!(),
        "/config" => {
            return Ok(builder
                .header("Content-Type", "application/json")
                .body((config.borrow() as &String).clone().into())
                .unwrap())
        }
        _ => req_path,
    });
    let mime = mime_guess::from_path(path);
    Ok(
        match UIAssets::get(
            path.strip_prefix("/")
                .unwrap_or(Path::new(INDEX))
                .as_os_str()
                .to_str()
                .unwrap(),
        ) {
            None => builder
                .status(StatusCode::NOT_FOUND)
                .body(UIAssets::get(INDEX).unwrap().data.into())
                .unwrap(),
            Some(file) => builder
                .status(StatusCode::OK)
                .header("Content-Type", mime.first_or_octet_stream().to_string())
                .body(file.data.into())
                .unwrap(),
        },
    )
}

pub async fn serve(addr: SocketAddr, config: KustodioConfiguration) -> Result<(), anyhow::Error> {
    for i in UIAssets::iter() {
        info!("{}", i);
    }
    let context = Arc::new(serde_json::to_string(&config)?);
    let service = make_service_fn(move |_| {
        let context = context.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| handle(req, context.clone()))) }
    });
    Server::bind(&addr).serve(service).await?;
    Ok(())
}
