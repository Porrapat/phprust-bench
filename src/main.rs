use rayon::prelude::*;

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

    let count = (2..5_000_000u64)
        .into_par_iter() // ใช้ทุก CPU core พร้อมกัน
        .filter(|&n| is_prime(n))
        .count();

    let elapsed = start.elapsed().as_secs_f64();
    println!("Rust (parallel) found {} primes in {:.3} seconds.", count, elapsed);
}
