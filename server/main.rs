use tonic::{transport::Server, Request, Response, Status};
use rayon::prelude::*;
use std::time::Instant;

pub mod sieve {
    tonic::include_proto!("sieve");
}

use sieve::sieve_service_server::{SieveService, SieveServiceServer};
use sieve::{SieveRequest, SieveResponse};

#[derive(Default)]
pub struct MySieveService {}

#[tonic::async_trait]
impl SieveService for MySieveService {
    async fn compute(
        &self,
        request: Request<SieveRequest>,
    ) -> Result<Response<SieveResponse>, Status> {
        let limit = request.get_ref().limit;
        println!("🧮 Received request for primes up to {}", limit);

        let start = Instant::now();
        let primes = sieve_parallel(limit);
        let elapsed = start.elapsed().as_secs_f64();

        let reply = SieveResponse {
            primes,
            elapsed,
        };

        println!("✅ Computation done in {:.3} sec", elapsed);
        Ok(Response::new(reply))
    }
}

fn sieve_parallel(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return vec![];
    }

    // ใช้ simple sieve ทีละ chunk แล้วรวมผล
    let sqrt_limit = (limit as f64).sqrt() as u64;
    let small_primes = sieve_basic(sqrt_limit);

    // แบ่งงานเป็นช่วง ๆ แล้วใช้ Rayon รันขนาน
    let chunk_size = 50_000;
    let chunks: Vec<(u64, u64)> = (2..=limit)
        .step_by(chunk_size as usize)
        .map(|start| {
            let end = (start + chunk_size - 1).min(limit);
            (start, end)
        })
        .collect();

    let primes: Vec<u64> = chunks
        .into_par_iter()
        .flat_map(|(start, end)| sieve_segment(start, end, &small_primes))
        .collect();

    primes
}

// 🔸 Basic sequential sieve สำหรับหาค่ารากฐาน
fn sieve_basic(limit: u64) -> Vec<u64> {
    let mut primes = vec![true; (limit + 1) as usize];
    for p in 2..=((limit as f64).sqrt() as u64) {
        if primes[p as usize] {
            let mut multiple = p * p;
            while multiple <= limit {
                primes[multiple as usize] = false;
                multiple += p;
            }
        }
    }
    primes
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
        .collect()
}

// 🔸 ทำงานในแต่ละ segment โดยใช้ small primes
fn sieve_segment(start: u64, end: u64, small_primes: &[u64]) -> Vec<u64> {
    let size = (end - start + 1) as usize;
    let mut is_prime = vec![true; size];

    for &p in small_primes {
        let mut multiple = ((start + p - 1) / p) * p;
        if multiple < p * p {
            multiple = p * p;
        }

        while multiple <= end {
            if multiple >= start {
                is_prime[(multiple - start) as usize] = false;
            }
            multiple += p;
        }
    }

    (0..size)
        .filter_map(|i| {
            if is_prime[i] {
                Some(start + i as u64)
            } else {
                None
            }
        })
        .collect()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let svc = MySieveService::default();

    println!("🚀 Starting gRPC server at {}", addr);
    println!("🧵 Rayon threads: {}", rayon::current_num_threads());

    Server::builder()
        .add_service(SieveServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
