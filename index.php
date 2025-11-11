<?php
function isPrime($n)
{
    if ($n < 2) {
        return false;
    }
    for ($i = 2; $i * $i <= $n; $i++) {
        if ($n % $i === 0) {
            return false;
        }
    }
    return true;
}

$limit = isset($_GET["limit"]) ? intval($_GET["limit"]) : 500000;

// Lock maximum input at 50,000,000
if ($limit > 50_000_000) {
    $limit = 50_000_000;
}

$start = microtime(true);
$count = 0;
for ($i = 2; $i < $limit; $i++) {
    if (isPrime($i)) {
        $count++;
    }
}
$elapsed = microtime(true) - $start;
echo "PHP found $count primes up to $limit in {$elapsed} seconds.";
