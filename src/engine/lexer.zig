const std = @import("std");

pub const TokenKind = enum {
    Atomic,
    LogicalAnd,
    LogicalOr,
    Pipe,
    EOF,
};

pub const Token = struct {
    kind: TokenKind,
    value: []const u8,
};

pub fn lex(allocator: std.mem.Allocator, source: []const u8) !std.ArrayList(Token) {
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
                    return error.InvalidChar;
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
    const allocator = std.testing.allocator;
    const tokens = try lex(allocator, "echo Hello World");
    defer tokens.deinit();

    var expected_tokens = std.ArrayList(Token).init(allocator);
    defer expected_tokens.deinit();

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
