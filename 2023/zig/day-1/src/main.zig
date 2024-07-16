const std = @import("std");
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

fn containsDigit(str: []const u8) bool {
    for (str) |char| {
        if (std.ascii.isDigit(char)) {
            return true;
        }
    }
    return false;
}

pub fn removeNonDigits(allocator: std.mem.Allocator, input: []const u8) ![]u8 {
    var result = std.ArrayList(u8).init(allocator);
    defer result.deinit();

    for (input) |char| {
        if (std.ascii.isDigit(char)) {
            try result.append(char);
        }
    }

    return result.toOwnedSlice();
}

pub fn part1(input: []const u8) !u32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var output_list = std.ArrayList(u8).init(std.heap.page_allocator);
    defer output_list.deinit();

    var line_iterator = std.mem.splitSequence(u8, input, "\n");

    while (line_iterator.next()) |line| {
        if (containsDigit(line)) {
            // std.debug.print("Line: {s}\n", .{line});
            const output = try removeNonDigits(allocator, line);
            defer allocator.free(output);

            // Create a buffer to hold the formatted string
            var buffer: [2]u8 = undefined;

            // Format the characters into the buffer
            const formatted = try std.fmt.bufPrint(&buffer, "{c}{c}", .{ output[0], output[output.len - 1] });

            // Parse the formatted string into an integer
            const result = try std.fmt.parseInt(u8, formatted, 10);
            try output_list.append(result);
        }
    }

    // Sum the values
    var sum: u32 = 0;
    for (output_list.items) |value| {
        sum += value;
    }

    return sum;
}

fn replaceString(allocator: std.mem.Allocator, input: []const u8, from: []const u8, to: []const u8) ![]u8 {
    var result = std.ArrayList(u8).init(allocator);
    defer result.deinit();

    var i: usize = 0;
    while (i < input.len) {
        if (std.mem.startsWith(u8, input[i..], from)) {
            try result.appendSlice(to);
            i += from.len;
        } else {
            try result.append(input[i]);
            i += 1;
        }
    }

    return result.toOwnedSlice();
}

pub fn part2(input: []const u8) !u32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var output_list = std.ArrayList(u8).init(std.heap.page_allocator);
    defer output_list.deinit();

    var line_iterator = std.mem.splitSequence(u8, input, "\n");

    while (line_iterator.next()) |line| {
        if (containsDigit(line)) {
            // std.debug.print("Line: {s}\n", .{line});
            var beans = try replaceString(allocator, line, "one", "1");
            // const output = try removeNonDigits(allocator, line);
            defer allocator.free(output);

            // Create a buffer to hold the formatted string
            var buffer: [2]u8 = undefined;

            // Format the characters into the buffer
            const formatted = try std.fmt.bufPrint(&buffer, "{c}{c}", .{ output[0], output[output.len - 1] });

            // Parse the formatted string into an integer
            const result = try std.fmt.parseInt(u8, formatted, 10);
            try output_list.append(result);
        }
    }

    // Sum the values
    var sum: u32 = 0;
    for (output_list.items) |value| {
        sum += value;
    }

    return sum;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const input = try std.fs.cwd().readFileAlloc(allocator, "src/puzzle-input.txt", 1024 * 1024);
    defer allocator.free(input);
    // const result = part1(input);
    const result = part2(input);
    std.debug.print("First: {!d}\n", .{result});
}

test "part1" {
    const test_input =
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;
    const result = try part1(test_input);
    try expect(result == 142);
    std.debug.print("{d}", .{result});
}

test "part2" {
    const test_input =
        \\two1nine
        \\eightwothree
        \\abcone2threexyz
        \\xtwone3four
        \\4nineeightseven2
        \\zoneight234
        \\7pqrstsixteen
    ;
    const result = try part1(test_input);
    try expect(result == 281);
    std.debug.print("{d}", .{result});
}
