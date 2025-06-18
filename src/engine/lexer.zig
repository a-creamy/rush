const std = @import("std");

var buffer: [1000]u8 = undefined;
var fba = std.heap.FixedBufferAllocator.init(&buffer);
const allocator = fba.allocator();

pub const TokenKind = enum {
    Atomic,
    Land,
    EOF,
};

pub const Token = struct {
    kind: TokenKind,
    value: []u8,
};

pub fn lex(source: []u8) !std.ArrayList(Token) {
    var tokens = std.ArrayList(Token).init(allocator);

    var start: usize = 0;
    while (start < source.len) {
        const current = source[start];

        switch (current) {
            'a'...'z', 'A'...'Z' => {
                const i = start;
                while (start < source.len and
                    ((source[start] >= 'a' and source[start] <= 'z') or
                        (source[start] >= 'A' and source[start] <= 'Z')))
                {
                    start += 1;
                }
                try tokens.append(Token{ .kind = TokenKind.Atomic, .value = source[i..start] });
            },
            '&' => {
                if (start + 1 < source.len and source[start + 1] == '&') {
                    try tokens.append(Token{ .kind = TokenKind.Land, .value = source[start .. start + 2] });
                    start += 2;
                } else {
                    return error.InvalidChar;
                }
            },
            ' ', '\n', '\t' => start += 1,
            else => {
                std.debug.print("flash Unknown Symbol: '{c}'\n", .{current});
                return error.InvalidChar;
            },
        }
    }

    try tokens.append(Token{ .kind = TokenKind.EOF, .value = @constCast(&[3]u8{ 'E', 'O', 'F' }) });
    return tokens;
}

const testing = std.testing;

test "lex atomic" {
    var source = "hello world".*;
    const tokens = try lex(&source);
    defer tokens.deinit();

    try testing.expect(tokens.items.len == 3);
    try testing.expect(tokens.items[0].kind == TokenKind.Atomic and tokens.items[1].kind == TokenKind.Atomic);
    try testing.expect(std.mem.eql(u8, tokens.items[0].value, "hello"));
    try testing.expect(std.mem.eql(u8, tokens.items[0].value, "world"));
    try testing.expect(tokens.items[2].kind == TokenKind.EOF);
}

test "lex land operator" {
    var source = "&&".*;
    const tokens = try lex(&source);
    defer tokens.deinit();

    try testing.expect(tokens.items.len == 2);
    try testing.expect(tokens.items[0].kind == TokenKind.Land);
    try testing.expect(std.mem.eql(u8, tokens.items[0].value, "&&"));
    try testing.expect(tokens.items[1].kind == TokenKind.EOF);
}

test "lex atomic with land operator" {
    var source = "hello && world".*;
    const tokens = try lex(&source);
    defer tokens.deinit();

    try testing.expect(tokens.items.len == 4);
    try testing.expect(tokens.items[0].kind == TokenKind.Atomic);
    try testing.expect(std.mem.eql(u8, tokens.items[0].value, "hello"));
    try testing.expect(tokens.items[1].kind == TokenKind.Land);
    try testing.expect(std.mem.eql(u8, tokens.items[1].value, "&&"));
    try testing.expect(tokens.items[2].kind == TokenKind.Atomic);
    try testing.expect(std.mem.eql(u8, tokens.items[2].value, "world"));
    try testing.expect(tokens.items[3].kind == TokenKind.EOF);
}

test "lex single ampersand returns error" {
    var source = "&".*;
    try testing.expectError(error.InvalidChar, lex(&source));
}

test "lex invalid character returns error" {
    var source = "hello123".*;
    try testing.expectError(error.InvalidChar, lex(&source));
}

test "lex special characters return error" {
    var source = "hello!".*;
    try testing.expectError(error.InvalidChar, lex(&source));
}
