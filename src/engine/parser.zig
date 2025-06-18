const std = @import("std");
const lexer = @import("lexer.zig");

const TokenKind = lexer.TokenKind;
const Token = lexer.Token;

const heap_allocator = std.heap.page_allocator;

pub const Operator = enum {
    land,
};

const Binary = struct {
    op: Operator,
    ll: *Expr,
    rr: *Expr,
};

const Type = enum {
    atomic,
    binary,
};

pub const Expr = union(Type) {
    atomic: std.ArrayList([]const u8),
    binary: Binary,
};

pub fn expression(tokens: []Token, cursor: *usize, precedence: u8) anyerror!Expr {
    var left = try primary(tokens, cursor);

    while (cursor.* < tokens.len) {
        const token = tokens[cursor.*];
        const token_precedence = get_precedence(token.kind);

        if (token_precedence < precedence or token.kind == TokenKind.EOF) {
            break;
        }

        cursor.* += 1;

        left = try infix(tokens, cursor, left, token, token_precedence);
    }

    return left;
}

fn primary(tokens: []Token, cursor: *usize) anyerror!Expr {
    var i = cursor.*;

    while (i < tokens.len and tokens[i].kind == TokenKind.Atomic) : (i += 1) {}

    var result = std.ArrayList([]const u8).init(heap_allocator);

    for (tokens[cursor.*..i]) |token| {
        try result.append(token.value);
    }

    cursor.* = i;

    return Expr{ .atomic = result };
}

fn infix(tokens: []Token, cursor: *usize, left: Expr, token: Token, precedence: u8) anyerror!Expr {
    const right = try expression(tokens, cursor, precedence + 1);

    const ll = try heap_allocator.create(Expr);
    const rr = try heap_allocator.create(Expr);
    ll.* = left;
    rr.* = right;

    return switch (token.kind) {
        TokenKind.Land => Expr{ .binary = Binary{ .op = Operator.land, .ll = ll, .rr = rr } },
        else => {
            std.debug.print("flash: Unknown Operator: '{}'", .{token.kind});
            return error.UnknownOperator;
        },
    };
}

fn get_precedence(kind: TokenKind) u8 {
    return switch (kind) {
        TokenKind.EOF, TokenKind.Atomic => 0,
        TokenKind.Land => 3,
    };
}

const testing = std.testing;

test "parse left-associative binary expression" {
    var tokens = [_]Token{
        Token{ .kind = TokenKind.Atomic, .value = "a" },
        Token{ .kind = TokenKind.Land, .value = "&&" },
        Token{ .kind = TokenKind.Atomic, .value = "b" },
        Token{ .kind = TokenKind.Land, .value = "&&" },
        Token{ .kind = TokenKind.Atomic, .value = "c" },
        Token{ .kind = TokenKind.EOF, .value = "EOF" },
    };

    var cursor: usize = 0;
    const expr = try expression(&tokens, &cursor, 0);

    try testing.expect(expr == .binary);
    try testing.expect(expr.binary.op == .land);
    try testing.expect(expr.binary.ll.* == .binary);
    try testing.expect(expr.binary.rr.* == .atomic);

    const left_binary = expr.binary.ll.binary;
    try testing.expect(left_binary.op == .land);
    try testing.expect(left_binary.ll.* == .atomic);
    try testing.expect(left_binary.rr.* == .atomic);
    try testing.expect(std.mem.eql(u8, left_binary.ll.atomic.items[0], "a"));
    try testing.expect(std.mem.eql(u8, left_binary.rr.atomic.items[0], "b"));
    try testing.expect(std.mem.eql(u8, expr.binary.rr.atomic.items[0], "c"));

    left_binary.ll.atomic.deinit();
    left_binary.rr.atomic.deinit();
    heap_allocator.destroy(left_binary.ll);
    heap_allocator.destroy(left_binary.rr);
    expr.binary.rr.atomic.deinit();
    heap_allocator.destroy(expr.binary.ll);
    heap_allocator.destroy(expr.binary.rr);
}

test "parse basic binary expression" {
    var tokens = [_]Token{
        Token{ .kind = TokenKind.Atomic, .value = "hello" },
        Token{ .kind = TokenKind.Atomic, .value = "world" },
        Token{ .kind = TokenKind.Land, .value = "&&" },
        Token{ .kind = TokenKind.Atomic, .value = "foo" },
        Token{ .kind = TokenKind.Atomic, .value = "bar" },
        Token{ .kind = TokenKind.EOF, .value = "EOF" },
    };

    var cursor: usize = 0;
    const expr = try expression(&tokens, &cursor, 0);

    try testing.expect(expr == .binary);
    try testing.expect(expr.binary.ll.atomic.items.len == 2);
    try testing.expect(std.mem.eql(u8, expr.binary.ll.atomic.items[0], "hello"));
    try testing.expect(std.mem.eql(u8, expr.binary.ll.atomic.items[1], "world"));
    try testing.expect(expr.binary.rr.atomic.items.len == 2);
    try testing.expect(std.mem.eql(u8, expr.binary.rr.atomic.items[0], "foo"));
    try testing.expect(std.mem.eql(u8, expr.binary.rr.atomic.items[1], "bar"));

    expr.binary.ll.atomic.deinit();
    expr.binary.rr.atomic.deinit();
    heap_allocator.destroy(expr.binary.ll);
    heap_allocator.destroy(expr.binary.rr);
}

test "parse empty token array" {
    var tokens = [_]Token{
        Token{ .kind = TokenKind.EOF, .value = "EOF" },
    };

    var cursor: usize = 0;
    const expr = try expression(&tokens, &cursor, 0);

    try testing.expect(expr == .atomic);
    try testing.expect(expr.atomic.items.len == 0);

    expr.atomic.deinit();
}

test "infix with unknown operator returns UnknownOperator error" {
    var left_expr = Expr{ .atomic = std.ArrayList([]const u8).init(heap_allocator) };
    try left_expr.atomic.append("test");

    const fake_token = Token{ .kind = TokenKind.Atomic, .value = "invalid" };
    var tokens = [_]Token{
        Token{ .kind = TokenKind.Atomic, .value = "right" },
        Token{ .kind = TokenKind.EOF, .value = "EOF" },
    };
    var cursor: usize = 0;

    const result = infix(&tokens, &cursor, left_expr, fake_token, 1);
    try testing.expectError(error.UnknownOperator, result);

    left_expr.atomic.deinit();
}

test "parse with cursor beyond token array bounds" {
    var tokens = [_]Token{
        Token{ .kind = TokenKind.Atomic, .value = "test" },
        Token{ .kind = TokenKind.EOF, .value = "EOF" },
    };

    var cursor: usize = 10;
    const expr = try expression(&tokens, &cursor, 0);

    try testing.expect(expr == .atomic);
    try testing.expect(expr.atomic.items.len == 0);
    try testing.expect(cursor == 10);

    expr.atomic.deinit();
}

test "primary with no atomic tokens" {
    var tokens = [_]Token{
        Token{ .kind = TokenKind.Land, .value = "&&" },
        Token{ .kind = TokenKind.EOF, .value = "EOF" },
    };

    var cursor: usize = 0;
    const expr = try primary(&tokens, &cursor);

    try testing.expect(expr == .atomic);
    try testing.expect(expr.atomic.items.len == 0);
    try testing.expect(cursor == 0);

    expr.atomic.deinit();
}
