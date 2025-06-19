const std = @import("std");

pub const lexer = @import("lexer.zig");
pub const parse = @import("parser.zig");

const Expr = parse.Expr;

pub fn eval(expr: *Expr) anyerror!void {
    switch (expr.*) {
        .atomic => |*atomic| {
            const allocator = std.heap.page_allocator;

            const args = atomic.toOwnedSlice() catch |err| {
                return err;
            };

            if (args.len == 0) {
                return;
            }

            var child = std.process.Child.init(args, allocator);
            child.stdout_behavior = .Inherit;
            child.stderr_behavior = .Inherit;

            child.spawn() catch |err| {
                return err;
            };

            const result = child.wait() catch |err| {
                return err;
            };

            if (result.Exited != 0) {
                return error.CommandFail;
            }
        },
        .binary => |*binary| {
            switch (binary.op) {
                .LogicalAnd => {
                    try eval(binary.ll);
                    try eval(binary.rr);
                },
                .LogicalOr => {
                    eval(binary.ll) catch {
                        try eval(binary.rr);
                    };
                },
            }
        },
    }
}
