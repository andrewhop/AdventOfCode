import cProfile
import pstats
import time
# import numpy as np

def location_to_num(location):
    part1 = ord("Z") - ord(location[0]) << 10
    part2 = ord("Z") - ord(location[1]) << 5
    part3 = ord("Z") - ord(location[2])
    result = part1 + part2 + part3
    return result


def parse_map(lines):
    result = [0] * (2 * 26426)
    # result = np.empty(2 * 26426, dtype=int)
    for line in lines:
        start, parts = [item.strip() for item in line.split("=")]
        left_num = location_to_num(parts[1:4])
        right_num = location_to_num(parts[6:9])
        this_num = location_to_num(start) * 2
        result[this_num] = left_num * 2
        result[this_num + 1] = right_num * 2
    return result


def update_node(map, instructions, instruction_length, step, current_index):
    left_or_right = instructions[step % instruction_length]
    next_node_index = map[current_index + left_or_right]
    return next_node_index

def parse_instruction(lines):
    instructions = lines.pop(0)
    instructions = [0 if char == 'L' else 1 for char in instructions]
    return instructions


def day8_p1(data):
    answer = 0
    lines = [x for x in data.split('\n')]
    instructions = parse_instruction(lines)
    instruction_length = len(instructions)
    lines.pop(0)
    mapping = parse_map(lines)

    location = location_to_num("AAA") * 2
    end = location_to_num("ZZZ")
    while location != end:
        location = update_node(mapping, instructions, instruction_length, answer, location)
        answer += 1
    return answer


def part_one(file):
    data = open(file).read().strip()
    return day8_p1(data)


def benchmark():
    data = open("input-1.txt").read().strip()
    day8_p1(data)
    day8_p1(data)
    day8_p1(data)

    start_time = time.time()
    for _ in range(100):
        day8_p1(data)
    end_time = time.time()
    time_taken = end_time - start_time
    print(f"Time taken to call the function 100 times: {time_taken / 100 * 1000 * 1000} microseconds")


def profile_function():
    data = open("input-1.txt").read().strip()
    profiler = cProfile.Profile()
    profiler.enable()
    for _ in range(1000):
        day8_p1(data)
    profiler.disable()

    # Save the profiling results to a file
    with open("profile_results.prof", "w") as f:
        ps = pstats.Stats(profiler, stream=f).sort_stats('cumulative')
        ps.print_stats()


if __name__ == '__main__':
    expected = 6
    got = part_one('sample-1.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input-1.txt')}")
    # benchmark()
    # profile_function()

    # expected = 6
    # got = part_two('sample-2.txt')
    # match = "match" if expected == got else "don't match"
    # print(f"Sample part 2 {match}, expected {expected}, got {got}")
    # print(f"part 2: {part_two('input-2.txt')}")
