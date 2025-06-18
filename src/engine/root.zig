const std = @import("std");

pub const lexer = @import("lexer.zig");
pub const parse = @import("parser.zig");

const Expr = parse.Expr;

pub fn eval(expr: *Expr) void {
    switch (expr.*) {
        .atomic => |*atomic| {
            const allocator = std.heap.page_allocator;

            const args = atomic.toOwnedSlice() catch |err| {
                std.debug.print("flash: Failed to convert atomic to slice: {}\n", .{err});
                return;
            };

            if (args.len == 0) {
                return;
            }

            var child = std.process.Child.init(args, allocator);
            child.stdout_behavior = .Inherit;
            child.stderr_behavior = .Inherit;

            child.spawn() catch |err| {
                std.debug.print("flash: Failed to spawn: '{s}' ({})\n", .{args[0], err});
                return;
            };

            _ = child.wait() catch |err| switch (err) {
                error.FileNotFound => {
                    std.debug.print("flash: Unknown Command: '{s}'\n", .{args[0]});
                },
                else => {
                    std.debug.print("flash: Failed to wait: '{s}' ({})\n", .{args[0], err});
                }
            };
        },
        .binary => |*binary| {
            switch (binary.op) {
                .land => {
                    eval(binary.ll);
                    eval(binary.rr);
                },
            }
        },
    }
}
