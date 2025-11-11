<?php
function sieve(int $limit): array {
    $sieve = array_fill(0, $limit + 1, true);
    $sieve[0] = $sieve[1] = false;

    for ($i = 2; $i * $i <= $limit; $i++) {
        if ($sieve[$i]) {
            for ($j = $i * $i; $j <= $limit; $j += $i) {
                $sieve[$j] = false;
            }
        }
    }

    $primes = [];
    foreach ($sieve as $num => $isPrime) {
        if ($isPrime) $primes[] = $num;
    }
    return $primes;
}

// -----------------------------

$limit = isset($_GET['limit']) ? (int)$_GET['limit'] : 500_000;
if ($limit > 50_000_000) $limit = 50_000_000;

$start = hrtime(true);
$primes = sieve($limit);
$end = hrtime(true);

$count = count($primes);
$elapsed = number_format(($end - $start) / 1e9, 6);

echo "PHP found $count primes up to $limit in {$elapsed} seconds.";

