package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"time"
)

func main() {
	// Parse args
	limit := uint64(500000) // default
	if len(os.Args) > 1 {
		if v, err := strconv.ParseUint(os.Args[1], 10, 64); err == nil {
			limit = v
		}
	}

	// Lock maximum input at 50,000,000
	if limit > 50_000_000 {
		limit = 50_000_000
	}

	start := time.Now()

	count := 0
	for n := uint64(2); n < limit; n++ {
		if isPrime(n) {
			count++
		}
	}

	elapsed := time.Since(start).Seconds()
	fmt.Printf("Go found %d primes up to %d in %.3f seconds.\n", count, limit, elapsed)
}

func isPrime(n uint64) bool {
	if n < 2 {
		return false
	}
	limit := uint64(math.Sqrt(float64(n))) + 1
	for i := uint64(2); i < limit; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}
