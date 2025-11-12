<?php
$limit = isset($_GET["limit"]) ? intval($_GET["limit"]) : 500000;

$start = microtime(true);

// Check OS
$isWindows = strtoupper(substr(PHP_OS, 0, 3)) === "WIN";

if ($isWindows) {
    $cmd = '.\\target\\release\\phprust-bench-client.exe ' . $limit;
} else {
    $cmd = "./target/release/phprust-bench-client " . $limit;
}

$start = microtime(true);
$output = shell_exec($cmd);
$elapsed = microtime(true) - $start;

echo nl2br("$output\nTotal call time from PHP: {$elapsed} seconds\n");
