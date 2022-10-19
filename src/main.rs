use axum::{extract::Query, routing::get, Router};
use std::env;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloData {
    #[serde(default)]
    name: String,
}

const DEFAULT_GREETING: &str = "Hello, world!";

// `String` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn hello(data: Query<HelloData>) -> String {
    if data.name.is_empty() {
        return String::from(DEFAULT_GREETING);
    }
    return format!("Hello, {}!", data.name);
}

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>(),
        _ => Ok(8080),
    }
    .unwrap();

    let app = Router::new().route("/hello", get(hello));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
