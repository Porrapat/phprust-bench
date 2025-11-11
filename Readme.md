# phprust-bench

A simple benchmark project to compare **PHP** and **Rust** performance in computing prime numbers.

This project demonstrates how PHP can call a compiled Rust binary via `shell_exec()` and measure total execution time, while Rust performs the heavy computation efficiently using all CPU cores via the **Rayon** library.

---

## 🚀 Overview

| Language | Method | Description |
|-----------|---------|--------------|
| **PHP** | Pure PHP | Calculates prime numbers using a simple loop and checks for divisibility. Single-threaded. |
| **Rust** | Binary (called via PHP) | Uses Rayon to compute in parallel across all available CPU cores. |

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
