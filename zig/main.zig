// zig build-exe main.zig -O ReleaseFast -fstrip -fsingle-threaded -femit-bin=phprust-bench.exe

const std = @import("std");
const math = std.math;

fn isPrime(n: u64) bool {
    if (n < 2) {
        return false;
    }
    const limit = @as(u64, @intFromFloat(@sqrt(@as(f64, @floatFromInt(n))))) + 1;
    var i: u64 = 2;
    while (i < limit) : (i += 1) {
        if (n % i == 0) {
            return false;
        }
    }
    return true;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Parse args
    var limit: u64 = 500000; // default
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len > 1) {
        limit = std.fmt.parseUnsigned(u64, args[1], 10) catch limit;
    }

    // Lock maximum input at 50,000,000
    if (limit > 50_000_000) {
        limit = 50_000_000;
    }

    const start = std.time.nanoTimestamp();

    var count: u64 = 0;
    var n: u64 = 2;
    while (n < limit) : (n += 1) {
        if (isPrime(n)) {
            count += 1;
        }
    }

    const end = std.time.nanoTimestamp();
    const elapsed = @as(f64, @floatFromInt(end - start)) / 1_000_000_000.0;

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Zig found {d} primes up to {d} in {d:.3} seconds.\n", .{ count, limit, elapsed });
}
