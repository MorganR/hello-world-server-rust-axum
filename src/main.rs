use axum::{
    error_handling::HandleError,
    extract::Query,
    http::StatusCode,
    routing::{get, get_service},
    Router, response::Html,
};
use serde::Deserialize;
use std::{env, time::Duration};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::compression::{CompressionLayer, predicate::SizeAbove};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
/// Query parameters for /strings/hello.
struct HelloData {
    #[serde(default)]
    name: String,
}

#[derive(Debug, Deserialize)]
/// Query parameters for /strings/lines.
struct LinesData {
    #[serde(default)]
    n: usize,
}

/// Query parameters for /math/power-reciprocals-alt.
type MathData = LinesData;

const DEFAULT_GREETING: &str = "Hello, world!";
const MAX_NAME_LEN: u16 = 500;

static SLOW_DURATION: Duration = Duration::from_millis(15);

/// Responds with a greeting based on the "name" paremeter.
async fn hello(data: Query<HelloData>) -> Result<String, StatusCode> {
    if data.name.is_empty() {
        return Ok(String::from(DEFAULT_GREETING));
    }
    if data.name.len() > usize::from(MAX_NAME_LEN) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(format!("Hello, {}!", data.name))
}

/// Responds with a greeting after a short delay.
async fn async_hello() -> Result<String, StatusCode> {
    tokio::time::sleep(SLOW_DURATION).await;
    Ok(String::from(DEFAULT_GREETING))
}

/// Responds with an ordered list of "n" items.
async fn lines(data: Query<LinesData>) -> Result<Html<String>, StatusCode> {
    let mut result = "<ol>\n".to_string();
    for i in 1..=(data.n) {
        result += &format!("  <li>Item number: {}</li>\n", i);
    }
    result += "</ol>";
    Ok(Html(result))
}

/// Computes a convergent sum with "n" terms.
async fn math_power_reciprocals_alt(data: Query<MathData>) -> Result<String, StatusCode> {
    let mut result = 0f64;
    let mut power = 0.5f64;
    let mut n = data.n;

    while n > 0 {
        power = power * 2.0;
        result += 1.0 / power;
        n -= 1;

        if n > 0 {
            power = power * 2.0;
            result -= 1.0 / power;
            n -= 1;
        }
    }

    Ok(result.to_string())
}

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>(),
        _ => Ok(8080),
    }
    .expect("PORT env var must be an int, or unset");

    // Define the strings routes, which all use compression.
    let strings = Router::new()
        .route("/hello", get(hello))
        .route("/async-hello", get(async_hello))
        .route("/lines", get(lines))
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(256)));

    let app = Router::new()
        .nest("/strings", strings)
        .route("/math/power-reciprocals-alt", get(math_power_reciprocals_alt))
        .nest_service(
            "/static",
            get_service(HandleError::new(
                ServeDir::new("static").precompressed_br(),
                |_| async move { (StatusCode::INTERNAL_SERVER_ERROR, String::from("I/O error")) },
            )),
        );

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        // Disable Nagle (see https://github.com/hyperium/hyper/issues/3187).
        .tcp_nodelay(true)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
