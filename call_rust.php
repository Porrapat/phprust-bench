<?php
$limit = isset($_GET["limit"]) ? intval($_GET["limit"]) : 500000;

$start = microtime(true);
$output = shell_exec('.\\target\\release\\phprust.exe' . " $limit");
$elapsed = microtime(true) - $start;

echo nl2br("$output\nTotal call time from PHP: {$elapsed} seconds\n");
