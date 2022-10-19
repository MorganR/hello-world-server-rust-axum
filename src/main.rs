use axum::{routing::get, Router};
use std::env;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn hello() -> &'static str {
    "Hello, world!"
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
