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

const ParseError = error{
    ExpectedAtomic,
    UnknownOperator,
    OutOfMemory,
};

pub fn expression(tokens: []Token, cursor: *usize, precedence: u8) ParseError!Expr {
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

fn primary(tokens: []Token, cursor: *usize) ParseError!Expr {
    var i = cursor.*;

    while (i < tokens.len and tokens[i].kind == TokenKind.Atomic) : (i += 1) {}

    var result = std.ArrayList([]const u8).init(heap_allocator);

    for (tokens[cursor.*..i]) |token| {
        try result.append(token.value);
    }

    cursor.* = i;

    return Expr{ .atomic = result };
}

fn infix(tokens: []Token, cursor: *usize, left: Expr, token: Token, precedence: u8) ParseError!Expr {
    const right = try expression(tokens, cursor, precedence + 1);

    const ll = try heap_allocator.create(Expr);
    const rr = try heap_allocator.create(Expr);
    ll.* = left;
    rr.* = right;

    return switch (token.kind) {
        TokenKind.Land => Expr{ .binary = Binary{ .op = Operator.land, .ll = ll, .rr = rr } },
        else => {
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
