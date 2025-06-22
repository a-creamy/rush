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
    atomic: *std.ArrayList([]const u8),
    binary: Binary,

    pub fn deinit(self: *Expr, allocator: std.mem.Allocator) void {
        switch (self.*) {
            .atomic => |list| {
                list.deinit();
            },
            .binary => |b| {
                b.ll.deinit(allocator);
                b.rr.deinit(allocator);
                allocator.destroy(b.ll);
                allocator.destroy(b.rr);
            },
        }
        allocator.destroy(self);
    }

    pub fn clone(self: *const Expr, allocator: std.mem.Allocator) !Expr {
        switch (self.*) {
            .atomic => |*atomic| {
                const new_atomic = try allocator.create(std.ArrayList([]const u8));
                new_atomic.* = try atomic.*.clone();
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

pub fn expression(tokens: []Token, allocator: std.mem.Allocator, cursor: *usize, precedence: u8) anyerror!Expr {
    var left = try primary(tokens, allocator, cursor);

    while (cursor.* < tokens.len) {
        const token = tokens[cursor.*];
        const token_precedence = get_precedence(token.kind);

        if (token_precedence < precedence or token_precedence == 0) {
            break;
        }

        cursor.* += 1;

        left = try infix(tokens, allocator, cursor, left, token, token_precedence);
    }

    return left;
}

fn primary(tokens: []Token, allocator: std.mem.Allocator, cursor: *usize) anyerror!Expr {
    var i = cursor.*;

    while (i < tokens.len and tokens[i].kind == TokenKind.Atomic) {
        i += 1;
    }

    var result = try allocator.create(std.ArrayList([]const u8));
    result.* = std.ArrayList([]const u8).init(allocator);

    for (tokens[cursor.*..i]) |token| {
        try result.append(token.value);
    }

    cursor.* = i;

    return Expr{ .atomic = result };
}

fn infix(tokens: []Token, allocator: std.mem.Allocator, cursor: *usize, left: Expr, token: Token, precedence: u8) anyerror!Expr {
    const right = try expression(tokens, allocator, cursor, precedence + 1);

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
