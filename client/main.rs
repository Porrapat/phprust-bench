use tonic::transport::{Channel, Endpoint};
use sieve::sieve_service_client::SieveServiceClient;
use sieve::SieveRequest;
use std::time::Duration;

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
    let limit = 1_000_000;
    // let server_addr = "http://192.168.1.10:50051"; // Fix IP ของ Rayon Server

    // println!("Start working here!");
    // let client_result = SieveServiceClient::connect(server_addr.to_string()).await;

    // let server_addr = "http://192.168.1.10:50051";
    let server_addr = "http://127.0.0.1:50051";

    println!("⏳ Checking connection...");

    let endpoint = Endpoint::from_shared(server_addr.to_string())
        .unwrap()
        .timeout(Duration::from_millis(300))
        .connect_timeout(Duration::from_millis(100));

    let client_result = endpoint.connect().await;

    let primes = match client_result {
        Ok(channel) => {
            println!("✅ Connected to server at {}", server_addr);

            // ✅ ต้อง wrap channel ก่อนใช้
            let mut client = SieveServiceClient::new(channel);

            let request = tonic::Request::new(SieveRequest { limit });
            match client.compute(request).await {
                Ok(response) => {
                    let resp = response.get_ref();
                    println!("Server computed {} primes in {:.3} sec", resp.primes.len(), resp.elapsed);
                    resp.primes.clone()
                }
                Err(err) => {
                    println!("⚠️ Server call failed: {}", err);
                    sieve_local(limit)
                }
            }
        }
        Err(err) => {
            println!("⚠️ Server unavailable ({}), using local sieve", err);
            sieve_local(limit)
        }
    };

    println!("Total primes found: {}", primes.len());
}
