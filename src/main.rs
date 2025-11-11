use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut limit: u64 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(500_000); // default

    // Lock maximum input at 50,000,000
    if limit > 50_000_000 {
        limit = 50_000_000;
    }

    let start = std::time::Instant::now();

    let count = (2..limit)
        .into_iter()
        .filter(|&n| is_prime(n))
        .count();

    let elapsed = start.elapsed().as_secs_f64();
    println!("Rust found {} primes up to {} in {:.3} seconds.", count, limit, elapsed);
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in 2..limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
