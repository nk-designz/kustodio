use http::StatusCode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rust_embed::RustEmbed;
use std::{convert::Infallible, net::SocketAddr, path::Path};

const INDEX: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "ui/target/build"]
struct UIAssets;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let builder = Response::builder();
    let req_path = req.uri().path();
    info!("Serving {}", req_path);
    let path = Path::new(if req_path == "/" {
        "/index.html"
    } else {
        req_path
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

pub async fn serve(addr: SocketAddr) -> Option<anyhow::Error> {
    for i in UIAssets::iter() {
        info!("{}", i);
    }
    let sf = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(sf);
    match server.await {
        Ok(_) => None,
        Err(err) => Some(anyhow::Error::new(err)),
    }
}
