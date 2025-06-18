const std = @import("std");
const engine = @import("engine/root.zig");

const lexer = engine.lexer;
const parse = engine.parse;

const Shell = struct {
    prompt: []const u8,

    fn new(prompt: []const u8) Shell {
        return Shell{ .prompt = prompt };
    }

    fn ask(self: Shell, buf: []u8) ![]u8 {
        const stdin = std.io.getStdIn().reader();
        const stdout = std.io.getStdOut().writer();

        _ = try stdout.print("{s}", .{self.prompt});

        const input = (try stdin.readUntilDelimiterOrEof(buf, '\n')) orelse {
            return error.Null;
        };

        return input;
    }
};

pub fn run() !void {
    const shell = Shell.new("> ");

    while (true) {
        var buf: [1024]u8 = undefined;
        const input = try shell.ask(&buf);

        const result = try lexer.lex(input);

        for (result.items) |token| {
            std.debug.print("kind: {}, value: {s}\n", .{ token.kind, token.value });
        }

        var cursor: usize = 0;
        var expr = try parse.expression(result.items, &cursor, 0);
        try print_expr(expr);

        try engine.eval(&expr);
    }
}

fn print_expr(expr: parse.Expr) !void {
    const stdout = std.io.getStdOut().writer();

    switch (expr) {
        .atomic => |val| {
            for (val.items) |result| {
                try stdout.print("atomic: {s}\n", .{result});
            }
        },
        .binary => |b| {
            try stdout.print("binary op: {any}\n", .{b.op});
            try print_expr(b.ll.*);
            try print_expr(b.rr.*);
        },
    }
}
