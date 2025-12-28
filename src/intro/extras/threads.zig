const std = @import("std");
const Thread = std.Thread;

fn f() void {
    std.debug.print("Hello from another thread\n", .{});
}

pub fn main() !void {
    const t1 = try Thread.spawn(.{}, f, .{});
    const t2 = try Thread.spawn(.{}, f, .{});

    std.debug.print("Hello from main thread\n", .{});

    t1.join();
    t2.join();
}
