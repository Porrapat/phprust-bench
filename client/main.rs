use tonic::transport::{Endpoint};
use sieve::sieve_service_client::SieveServiceClient;
use sieve::SieveRequest;
use std::time::Duration;
use std::time::Instant;
use dotenvy::dotenv;
use std::env;

pub mod sieve {
    tonic::include_proto!("sieve");
}

fn sieve_local(limit: u64) -> Vec<u64> {
    let mut primes = vec![true; (limit + 1) as usize];
    let mut result = Vec::new();
    for p in 2..=limit {
        if primes[p as usize] {
            result.push(p);
            let mut multiple = p * p;
            while multiple <= limit {
                primes[multiple as usize] = false;
                multiple += p;
            }
        }
    }
    result
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    let limit: u64 = env::var("LIMIT").unwrap_or_else(|_| "1000000".to_string()).parse().unwrap();

    println!("⏳ Checking connection...");

    let endpoint = Endpoint::from_shared(server_addr.to_string())
        .unwrap()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_millis(100));

    let client_result = endpoint.connect().await;

    println!("🧮 Received request for primes up to n = {}", limit);

    match client_result {
        Ok(channel) => {
            println!("✅ Connected to server at {}", server_addr);

            // ✅ ต้อง wrap channel ก่อนใช้
            let mut client = SieveServiceClient::new(channel);

            let request = tonic::Request::new(SieveRequest { limit });
            match client.compute(request).await {
                Ok(response) => {
                    let resp = response.get_ref();
                    println!(
                        "Server computed n = {} → {} primes in {:.3} sec",
                        limit, resp.count, resp.elapsed
                    );
                    vec![] // ถ้าไม่ต้องใช้ primes จริง
                }
                Err(err) => {
                    // println!("⚠️ Server call failed: {}", err);
                    // sieve_local(limit)
                    println!("⚠️ Server call failed: {}, using local sieve", err);
                    let start = Instant::now();
                    let local = sieve_local(limit);
                    let elapsed = start.elapsed().as_secs_f64();
                    println!("🧮 Local computed {} primes in {:.3} sec", local.len(), elapsed);
                    local
                }
            }
        }
        Err(err) => {
            // println!("⚠️ Server unavailable ({}), using local sieve", err);
            // sieve_local(limit)
            println!("⚠️ Server call failed: {}, using local sieve", err);
            let start = Instant::now();
            let local = sieve_local(limit);
            let elapsed = start.elapsed().as_secs_f64();
            println!("🧮 Local computed {} primes in {:.3} sec", local.len(), elapsed);
            local
        }
    };
}
