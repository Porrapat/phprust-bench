package main

import (
	"fmt"
	"math"
	"net/http"
	"strconv"
	"time"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()

	r.GET("/", func(c *gin.Context) {

		// Parse query: ?limit=500000
		limit := uint64(500000)
		if v := c.Query("limit"); v != "" {
			if parsed, err := strconv.ParseUint(v, 10, 64); err == nil {
				limit = parsed
			}
		}

		// Safety cap
		if limit > 50_000_000 {
			limit = 50_000_000
		}

		start := time.Now()
		count := primeCount(limit)
		elapsed := time.Since(start).Seconds()

		c.String(http.StatusOK,
			fmt.Sprintf("Go (Gin) found %d primes up to %d in %.3f seconds.\n",
				count, limit, elapsed))
	})

	// Listen on port 8082
	r.Run(":8082")
}

func primeCount(limit uint64) int {
	count := 0
	for n := uint64(2); n < limit; n++ {
		if isPrime(n) {
			count++
		}
	}
	return count
}

func isPrime(n uint64) bool {
	if n < 2 {
		return false
	}
	max := uint64(math.Sqrt(float64(n))) + 1
	for i := uint64(2); i < max; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}
