use axum::{
    extract::Query,
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // สร้าง router
    let app = Router::new().route("/", get(handler));

    // Port 8081
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("Running Axum server on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Params {
    limit: Option<u64>,
}

async fn handler(Query(params): Query<Params>) -> Html<String> {
    // default = 500000
    let mut limit = params.limit.unwrap_or(500_000);

    // cap at 50,000,000
    if limit > 50_000_000 {
        limit = 50_000_000;
    }

    let start = std::time::Instant::now();

    let count = (2..limit).filter(|&n| is_prime(n)).count();

    let elapsed = start.elapsed().as_secs_f64();

    // Output เหมือน CLI เป๊ะ
    let body = format!(
        "Rust found {} primes up to {} in {:.3} seconds.",
        count, limit, elapsed
    );

    Html(body)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in 2..limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
