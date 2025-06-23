const std = @import("std");

pub const lexer = @import("lexer.zig");
pub const parse = @import("parser.zig");

const Expr = parse.Expr;

pub fn eval(expr: *Expr, allocator: std.mem.Allocator) anyerror!void {
    switch (expr.*) {
        .atomic => |args| {
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
                    try eval(binary.ll, allocator);
                    try eval(binary.rr, allocator);
                },
                .LogicalOr => {
                    eval(binary.ll, allocator) catch {
                        try eval(binary.rr, allocator);
                    };
                },
                .Pipe => {
                    const fd = try std.posix.pipe();

                    const first_command = try extract_args(binary.ll);
                    const second_command = try extract_args(binary.rr);

                    const pid = try std.posix.fork();
                    switch (pid) {
                        0 => {
                            try runPipe(fd, first_command, second_command, allocator);
                        },
                        else => {},
                    }
                },
                .Separator => {
                    eval(binary.ll, allocator) catch |err| {
                        std.debug.print("flash: Eval: {}\n", .{err});
                    };
                    eval(binary.rr, allocator) catch |err| {
                        std.debug.print("flash: Eval: {}\n", .{err});
                    };
                },
            }
        },
    }
}

fn extract_args(expr: ?*Expr) ![][]const u8 {
    if (expr) |e| {
        switch (e.*) {
            .atomic => |atomic| return atomic,
            else => return error.ExpectedAtomic,
        }
    } else {
        return error.NullPtr;
    }
}

fn runPipe(pfd: [2]i32, first_command: [][]const u8, second_command: [][]const u8, allocator: std.mem.Allocator) !void {
    const pid = try std.posix.fork();

    switch (pid) {
        0 => {
            // Child process for the first command
            try std.posix.dup2(pfd[1], 1); // Redirect stdout to the pipe
            std.posix.close(pfd[0]); // Close unused read end
            // Execute the first command passed from main
            std.process.execve(allocator, first_command, null) catch {};
        },
        else => {
            // Child process for the second command
            try std.posix.dup2(pfd[0], 0); // Redirect stdin from the pipe
            std.posix.close(pfd[1]); // Close unused write end
            // Execute the second command passed from main
            std.process.execve(allocator, second_command, null) catch {};
        },
    }
}

test {
    @import("std").testing.refAllDecls(@This());
}
