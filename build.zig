const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const engine_module = b.addModule("engine", .{
        .root_source_file = b.path("src/engine/root.zig"),
    });

    const exe = b.addExecutable(.{
        .name = "flash",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.root_module.addImport("engine", engine_module);

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const engine_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/engine/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_engine_unit_tests = b.addRunArtifact(engine_unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_engine_unit_tests.step);
}
