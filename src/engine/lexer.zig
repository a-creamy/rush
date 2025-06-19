const std = @import("std");

var buffer: [1000]u8 = undefined;
var fba = std.heap.FixedBufferAllocator.init(&buffer);
const allocator = fba.allocator();

pub const TokenKind = enum {
    Atomic,
    LogicalAnd,
    LogicalOr,
    Pipe,
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
            '&' => {
                if (start + 1 < source.len and source[start + 1] == '&') {
                    try tokens.append(Token{ .kind = TokenKind.LogicalAnd, .value = source[start .. start + 2] });
                    start += 2;
                } else {
                    return error.InvalidChar;
                }
            },
            '|' => {
                if (start + 1 < source.len and source[start + 1] == '|') {
                    try tokens.append(Token{ .kind = TokenKind.LogicalOr, .value = source[start .. start + 2] });
                    start += 2;
                } else {
                    try tokens.append(Token{ .kind = TokenKind.Pipe, .value = source[start .. start + 1] });
                }
            },
            ' ', '\n', '\t' => start += 1,
            else => {
                const i = start;
                while (start < source.len and
                    ((source[start] >= 'a' and source[start] <= 'z') or
                        (source[start] >= 'A' and source[start] <= 'Z') or
                        (source[start] >= '0' and source[start] <= '9') or
                        source[start] == '_' or source[start] == '-' or source[start] == '.' or source[start] == '/'))
                {
                    start += 1;
                }
                try tokens.append(Token{ .kind = TokenKind.Atomic, .value = source[i..start] });
            },
        }
    }

    try tokens.append(Token{ .kind = TokenKind.EOF, .value = @constCast(&[3]u8{ 'E', 'O', 'F' }) });
    return tokens;
}
