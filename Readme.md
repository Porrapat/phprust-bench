# phprust-bench

A simple benchmark project to compare **PHP** and **Rust** performance in computing prime numbers.

This project demonstrates how PHP can call a compiled Rust binary via `shell_exec()` and measure total execution time, while Rust performs the heavy computation efficiently using all CPU cores via the **Rayon** library.

---
## 🔗 Other languages
[ภาษาไทย (Readme_th.md)](Readme_th.md)

---

## 🚀 Overview

| Language | Method | Description |
|-----------|---------|--------------|
| **PHP** | Pure PHP | Calculates prime numbers using a simple loop and checks for divisibility. Single-threaded. |
| **Rust** | Binary (called via PHP) | Uses Rayon to compute in parallel across all available CPU cores. |

---

## 🌐 Live Demo

You can view the live demo here:

- **Main page:** [https://phprust-bench.porrapat.com/](https://phprust-bench.porrapat.com/)
- **Rust call example:** [https://phprust-bench.porrapat.com/call_rust.php](https://phprust-bench.porrapat.com/call_rust.php)

### 💻 Server Specs
This project runs on the **smallest Droplet size** from **DigitalOcean (region: SGP1)**:

```
PHP-Rust-Bench
512 MB RAM / 10 GB Disk / SGP1 - Ubuntu 25.10 x64
```

You can also test the benchmark with a custom limit by appending a `limit` parameter (default = 500,000):

```bash
https://phprust-bench.porrapat.com/?limit=100000
https://phprust-bench.porrapat.com/?limit=400000
https://phprust-bench.porrapat.com/?limit=800000
https://phprust-bench.porrapat.com/call_rust.php?limit=100000
https://phprust-bench.porrapat.com/call_rust.php?limit=400000
https://phprust-bench.porrapat.com/call_rust.php?limit=800000
https://phprust-bench.porrapat.com/call_rust.php?limit=2000000
```

> The limit defines the upper bound of numbers to check for primes.

The PHP is capped at **2,000,000** but Rust is at **20,000,000** to prevent memory overflow or performance degradation on smaller servers.

---

## 🔧 Build & Run

### 1. Build Rust binary

```bash
cargo build --release
```

This produces `target/release/phprust-bench` (or `.exe` on Windows).

### 2. Run from PHP

We locked output at 2,000,000 to protect PHP server. Otherwise for Rust is locked at 20,000,000. You can uncommented there.

---

## 🧠 Example Output

```
Rust found 148933 primes up to 2000000 in 0.108 seconds.
Total call time from PHP: 0.154 seconds.
```

---

## 🧩 Cross-Platform Support

- ✅ Windows: uses `.exe` path (escaped with `\\`)
- ✅ Linux / macOS: uses `./target/release/phprust-bench`

---

## 🧱 License

MIT License © 2025 Porrapat Petchdamrongskul
