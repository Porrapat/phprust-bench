<?php
function isPrime($n) {
    if ($n < 2) return false;
    for ($i = 2; $i * $i <= $n; $i++) {
        if ($n % $i === 0) return false;
    }
    return true;
}

$start = microtime(true);

$count = 0;
for ($i = 2; $i < 5_000_000; $i++) {
    if (isPrime($i)) {
        $count++;
    }
}

$elapsed = microtime(true) - $start;
echo "PHP found $count primes in {$elapsed} seconds.\n";
