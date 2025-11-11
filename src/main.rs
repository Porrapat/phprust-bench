use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut limit: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(500_000); // Use usize for indexing

    // Lock maximum input at 20,000,000
    if limit > 20_000_000 {
        limit = 20_000_000;
    }

    let start = std::time::Instant::now();

    let count = count_primes(limit as u64);

    let elapsed = start.elapsed().as_secs_f64();
    println!(
        "Rust found {} primes up to {} in {:.3} seconds.",
        count, limit, elapsed
    );
}

fn count_primes(limit: u64) -> usize {
    if limit < 2 {
        return 0;
    }
    let limit_usize = limit as usize;
    let mut is_prime = vec![true; limit_usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for multiple in (i * i..=limit_usize).step_by(i) {
                is_prime[multiple] = false;
            }
        }
    }
    is_prime.iter().filter(|&&p| p).count()
}
