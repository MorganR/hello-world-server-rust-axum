use axum::{
    error_handling::HandleError,
    extract::Query,
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::{env, time::Duration};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::compression::{CompressionLayer, predicate::SizeAbove};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct HelloData {
    #[serde(default)]
    name: String,
}

#[derive(Debug, Deserialize)]
struct MathData {
    #[serde(default)]
    n: u64,
}

// Re-use MathData since their needs are equivalent.
type LinesData = MathData;

const DEFAULT_GREETING: &str = "Hello, world!";
const MAX_NAME_LEN: u16 = 500;

static SLOW_DURATION: Duration = Duration::from_millis(15);

async fn hello(data: Query<HelloData>) -> Result<String, StatusCode> {
    if data.name.is_empty() {
        return Ok(String::from(DEFAULT_GREETING));
    }
    if data.name.len() > usize::from(MAX_NAME_LEN) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(format!("Hello, {}!", data.name))
}

async fn async_hello() -> Result<String, StatusCode> {
    tokio::time::sleep(SLOW_DURATION).await;
    Ok(String::from(DEFAULT_GREETING))
}

async fn lines(data: Query<LinesData>) -> Result<String, StatusCode> {
    let mut result = "<ol>\n".to_string();
    for i in 1..=(data.n) {
        result += "  <li>Item number: ";
        result += &i.to_string();
        result += "</li>\n";
    }
    result += "</ol>";
    Ok(result)
}

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

    let cpus = num_cpus::get();
    println!("Running with {} cpus", cpus);

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
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
