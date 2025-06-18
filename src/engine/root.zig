const std = @import("std");

pub const lexer = @import("lexer.zig");
pub const parse = @import("parser.zig");

const Expr = parse.Expr;

pub fn eval(expr: *Expr) !void {
    return switch (expr.*) {
        .atomic => |*atomic| {
            const allocator = std.heap.page_allocator;
            
            const args = try atomic.toOwnedSlice();
            var child = std.process.Child.init(args, allocator);

            child.stdout_behavior = .Inherit;
            child.stderr_behavior = .Inherit;

            try child.spawn();
            _ = try child.wait();
        },
        .binary => |*binary| {
            switch (binary.op) {
                .land => {
                    eval(binary.ll) catch {
                        return;
                    };
                    try eval(binary.rr);
                }
            }
        }
    };
}
