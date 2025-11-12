use tonic::transport::Channel;
use sieve::sieve_service_client::SieveServiceClient;
use sieve::SieveRequest;

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
    let server_addr = "http://192.168.1.10:50051"; // Fix IP ของ Rayon Server

    println!("Start working here!");
    let client_result = SieveServiceClient::connect(server_addr.to_string()).await;

    let primes = match client_result {
        Ok(mut client) => {
            println!("✅ Connected to server at {}", server_addr);
            let request = tonic::Request::new(SieveRequest { limit });
            match client.compute(request).await {
                Ok(response) => {
                    println!("Server computed {} primes in {:.3} sec",
                        response.get_ref().primes.len(),
                        response.get_ref().elapsed);
                    response.get_ref().primes.clone()
                }
                Err(_) => {
                    println!("⚠️ Server call failed — using local sieve");
                    sieve_local(limit)
                }
            }
        }
        Err(_) => {
            println!("⚠️ Server unavailable — using local sieve");
            sieve_local(limit)
        }
    };

    println!("Total primes found: {}", primes.len());
}
