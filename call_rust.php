<?php
$start = microtime(true);
$output = shell_exec('.\\target\\release\\phprust.exe');
$elapsed = microtime(true) - $start;

echo "Rust program said:\n$output\n";
echo "Total call time from PHP: {$elapsed} seconds\n";
