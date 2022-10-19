use axum::{extract::Query, handler::Handler, http::StatusCode, routing::get, Router};
use serde::Deserialize;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::compression::CompressionLayer;

#[derive(Debug, Deserialize)]
struct HelloData {
    #[serde(default)]
    name: String,
}

const DEFAULT_GREETING: &str = "Hello, world!";
const MAX_NAME_LEN: u16 = 500;

async fn hello(data: Query<HelloData>) -> Result<String, StatusCode> {
    if data.name.is_empty() {
        return Ok(String::from(DEFAULT_GREETING));
    }
    if data.name.len() > usize::from(MAX_NAME_LEN) {
        return Err(StatusCode::BAD_REQUEST);
    }
    return Ok(format!("Hello, {}!", data.name));
}

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>(),
        _ => Ok(8080),
    }
    .unwrap();

    let app = Router::new().route("/hello", get(hello.layer(CompressionLayer::new())));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
