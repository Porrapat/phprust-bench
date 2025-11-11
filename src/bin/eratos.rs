use std::time::Instant;

fn sieve(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit + 1];
    sieve[0] = false;
    sieve[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if sieve[p] {
            let mut j = p * p;
            while j <= limit {
                sieve[j] = false;
                j += p;
            }
        }
        p += 1;
    }

    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

fn main() {
    let limit: usize = std::env::args()
        .nth(1)
        .and_then(|x| x.parse().ok())
        .unwrap_or(500_000);

    let limit = limit.min(500_000_000);

    let start = Instant::now();
    let primes = sieve(limit);
    let elapsed = start.elapsed().as_secs_f64();

    println!(
        "Rust found {} primes up to {} in {:.6} seconds.",
        primes.len(),
        limit,
        elapsed
    );
}
