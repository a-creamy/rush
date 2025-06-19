const std = @import("std");
const engine = @import("engine");

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

        const result = lexer.lex(input) catch |err| {
            std.debug.print("flash: l{}\n", .{err});
            continue;
        };

        var cursor: usize = 0;
        var expr = parse.expression(result.items, &cursor, 0) catch |err| {
            std.debug.print("flash: p{}\n", .{err});
            continue;
        };

        engine.eval(&expr) catch |err| {
            std.debug.print("flash: e{}\n", .{err});
        };
    }
}
