const std = @import("std");
const activeTag = std.meta.activeTag;
const lexer = @import("lexer.zig");

const TokenKind = lexer.TokenKind;
const Token = lexer.Token;

pub const Operator = enum {
    LogicalAnd,
    LogicalOr,
    Pipe,
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
    atomic: [][]const u8,
    binary: Binary,

    pub fn deinit(self: *Expr, allocator: std.mem.Allocator) void {
        switch (self.*) {
            .atomic => |a| {
                allocator.free(a);
            },
            .binary => |b| {
                b.ll.deinit(allocator);
                b.rr.deinit(allocator);
                allocator.destroy(b.ll);
                allocator.destroy(b.rr);
            },
        }
    }

    pub fn clone(self: *const Expr, allocator: std.mem.Allocator) !Expr {
        switch (self.*) {
            .atomic => |*atomic| {
                const new_atomic = atomic.*;
                return Expr{ .atomic = new_atomic };
            },
            .binary => |binary| {
                const ll = try allocator.create(Expr);

                const rr = try allocator.create(Expr);

                ll.* = try binary.ll.clone(allocator);
                rr.* = try binary.rr.clone(allocator);

                return Expr{
                    .binary = Binary{
                        .op = binary.op,
                        .ll = ll,
                        .rr = rr,
                    },
                };
            },
        }
    }
};

pub fn expression(tokens: []Token, cursor: *usize, precedence: u8, allocator: std.mem.Allocator) anyerror!Expr {
    var left = try primary(tokens, cursor, allocator);

    while (cursor.* < tokens.len) {
        const token = tokens[cursor.*];
        const token_precedence = get_precedence(token.kind);

        if (token_precedence < precedence or token_precedence == 0) {
            break;
        }

        cursor.* += 1;

        left = try infix(tokens, cursor, left, token, token_precedence, allocator);
    }

    return left;
}

fn primary(tokens: []Token, cursor: *usize, allocator: std.mem.Allocator) anyerror!Expr {
    var i = cursor.*;

    while (i < tokens.len and tokens[i].kind == TokenKind.Atomic) {
        i += 1;
    }

    var result = std.ArrayList([]const u8).init(allocator);
    defer result.deinit();

    for (tokens[cursor.*..i]) |token| {
        try result.append(token.value);
    }

    cursor.* = i;

    return Expr{ .atomic = try result.toOwnedSlice() };
}

fn infix(tokens: []Token, cursor: *usize, left: Expr, token: Token, precedence: u8, allocator: std.mem.Allocator) anyerror!Expr {
    const right = try expression(tokens, cursor, precedence + 1, allocator);

    const ll = try allocator.create(Expr);

    const rr = try allocator.create(Expr);

    ll.* = try left.clone(allocator);
    rr.* = try right.clone(allocator);

    return switch (token.kind) {
        TokenKind.LogicalAnd => Expr{ .binary = Binary{ .op = Operator.LogicalAnd, .ll = ll, .rr = rr } },
        TokenKind.LogicalOr => Expr{ .binary = Binary{ .op = Operator.LogicalOr, .ll = ll, .rr = rr } },
        TokenKind.Pipe => Expr{ .binary = Binary{ .op = Operator.Pipe, .ll = ll, .rr = rr } },
        else => {
            return error.UnknownOperator;
        },
    };
}

fn get_precedence(kind: TokenKind) u8 {
    return switch (kind) {
        TokenKind.EOF, TokenKind.Atomic => 0,
        TokenKind.LogicalAnd, TokenKind.LogicalOr => 3,
        TokenKind.Pipe => 4,
    };
}

test "Parse Atomic" {
    const allocator = std.testing.allocator;

    const tokens = try lexer.lex("echo Hello World", allocator);
    defer tokens.deinit();

    var cursor: usize = 0;
    var expr = try expression(tokens.items, &cursor, 0, allocator);
    defer expr.deinit(allocator);

    var list = std.ArrayList([]const u8).init(allocator);
    defer list.deinit();

    try list.append("echo");
    try list.append("Hello");
    try list.append("World");

    var expected_expr = Expr{
        .atomic = try list.toOwnedSlice(),
    };
    defer expected_expr.deinit(allocator);

    try std.testing.expect(expected_expr.atomic.len == expr.atomic.len);

    for (0..expr.atomic.len) |i| {
        try std.testing.expectEqualStrings(expected_expr.atomic[i], expr.atomic[i]);
    }
}

test "Parse a binary" {
    const allocator = std.testing.allocator;

    const tokens = try lexer.lex("echo Hello World && echo Hi", allocator);
    defer tokens.deinit();

    var cursor: usize = 0;
    var expr = try expression(tokens.items, &cursor, 0, allocator);
    defer expr.deinit(allocator);

    try std.testing.expect(expr.binary.op == Operator.LogicalAnd);
    try std.testing.expect(expr.binary.ll.atomic.len == 3);
    try std.testing.expectEqualStrings(expr.binary.ll.atomic[0], "echo");
    try std.testing.expectEqualStrings(expr.binary.ll.atomic[1], "Hello");
    try std.testing.expectEqualStrings(expr.binary.ll.atomic[2], "World");

    try std.testing.expect(expr.binary.rr.atomic.len == 2);
    try std.testing.expectEqualStrings(expr.binary.rr.atomic[0], "echo");
    try std.testing.expectEqualStrings(expr.binary.rr.atomic[1], "Hi");
}
