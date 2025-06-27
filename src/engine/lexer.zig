const std = @import("std");

pub const TokenKind = enum {
    Atomic,
    LogicalAnd,
    LogicalOr,
    Pipe,
    Separator,
    Background,
    EOF,
};

pub const Token = struct {
    kind: TokenKind,
    value: []const u8,
};

pub fn lex(source: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Token) {
    var tokens = std.ArrayList(Token).init(allocator);

    var index: usize = 0;
    while (index < source.len) {
        const ch = source[index];

        switch (ch) {
            '&' => {
                if (index + 1 < source.len and source[index + 1] == '&') {
                    try tokens.append(Token{ .kind = TokenKind.LogicalAnd, .value = source[index .. index + 2] });
                    index += 2;
                } else {
                    try tokens.append(Token{ .kind = TokenKind.Background, .value = source[index .. index + 1] });
                    index += 1;
                }
            },
            '|' => {
                if (index + 1 < source.len and source[index + 1] == '|') {
                    try tokens.append(Token{ .kind = TokenKind.LogicalOr, .value = source[index .. index + 2] });
                    index += 2;
                } else {
                    try tokens.append(Token{ .kind = TokenKind.Pipe, .value = source[index .. index + 1] });
                    index += 1;
                }
            },
            ';' => {
                try tokens.append(Token{
                    .kind = TokenKind.Separator,
                    .value = source[index .. index + 1],
                });
                index += 1;
            },
            ' ', '\n', '\t' => index += 1,
            else => {
                const i = index;
                while (index < source.len and
                    ((source[index] >= 'a' and source[index] <= 'z') or
                        (source[index] >= 'A' and source[index] <= 'Z') or
                        (source[index] >= '0' and source[index] <= '9') or
                        source[index] == '_' or source[index] == '-' or source[index] == '.' or source[index] == '/'))
                {
                    index += 1;
                }
                try tokens.append(Token{ .kind = TokenKind.Atomic, .value = source[i..index] });
            },
        }
    }

    try tokens.append(Token{ .kind = TokenKind.EOF, .value = "" });
    return tokens;
}

test "Lex Atomic" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const tokens = try lex("echo Hello World", allocator);

    var expected_tokens = std.ArrayList(Token).init(allocator);

    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "echo" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "Hello" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "World" });
    try expected_tokens.append(Token{ .kind = TokenKind.EOF, .value = "" });

    try std.testing.expect(expected_tokens.items.len == tokens.items.len);

    for (0..tokens.items.len) |i| {
        try std.testing.expect(std.mem.eql(u8, expected_tokens.items[i].value, tokens.items[i].value));
        try std.testing.expect(expected_tokens.items[i].kind == tokens.items[i].kind);
    }
}

test "Lex Single Char Operator" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const tokens = try lex("ls | grep a", allocator);

    var expected_tokens = std.ArrayList(Token).init(allocator);

    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "ls" });
    try expected_tokens.append(Token{ .kind = TokenKind.Pipe, .value = "|" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "grep" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "a" });
    try expected_tokens.append(Token{ .kind = TokenKind.EOF, .value = "" });

    try std.testing.expect(expected_tokens.items.len == tokens.items.len);

    for (0..tokens.items.len) |i| {
        try std.testing.expect(std.mem.eql(u8, expected_tokens.items[i].value, tokens.items[i].value));
        try std.testing.expect(expected_tokens.items[i].kind == tokens.items[i].kind);
    }
}

test "Lex Double Char Operator" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const tokens = try lex("ls || grep a", allocator);

    var expected_tokens = std.ArrayList(Token).init(allocator);

    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "ls" });
    try expected_tokens.append(Token{ .kind = TokenKind.LogicalOr, .value = "||" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "grep" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "a" });
    try expected_tokens.append(Token{ .kind = TokenKind.EOF, .value = "" });

    try std.testing.expect(expected_tokens.items.len == tokens.items.len);

    for (0..tokens.items.len) |i| {
        try std.testing.expect(std.mem.eql(u8, expected_tokens.items[i].value, tokens.items[i].value));
        try std.testing.expect(expected_tokens.items[i].kind == tokens.items[i].kind);
    }
}

test "Lex Multiple Operator's" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const tokens = try lex("ls | grep a | tr a-z A-Z || echo Failed", allocator);

    var expected_tokens = std.ArrayList(Token).init(allocator);

    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "ls" });
    try expected_tokens.append(Token{ .kind = TokenKind.Pipe, .value = "|" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "grep" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "a" });
    try expected_tokens.append(Token{ .kind = TokenKind.Pipe, .value = "|" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "tr" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "a-z" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "A-Z" });
    try expected_tokens.append(Token{ .kind = TokenKind.LogicalOr, .value = "||" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "echo" });
    try expected_tokens.append(Token{ .kind = TokenKind.Atomic, .value = "Failed" });

    try expected_tokens.append(Token{ .kind = TokenKind.EOF, .value = "" });

    try std.testing.expect(expected_tokens.items.len == tokens.items.len);

    for (0..tokens.items.len) |i| {
        try std.testing.expect(std.mem.eql(u8, expected_tokens.items[i].value, tokens.items[i].value));
        try std.testing.expect(expected_tokens.items[i].kind == tokens.items[i].kind);
    }
}
