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

fn main() {
    let start = std::time::Instant::now();

    let mut count = 0u64;
    for i in 2..5_000_000 {
        if is_prime(i) {
            count += 1;
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!("Rust found {} primes in {:.3} seconds.", count, elapsed);
}
