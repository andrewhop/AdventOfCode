import math
import time


def parse_map(lines):
    result = {}
    for line in lines:
        start, parts = [item.strip() for item in line.split("=")]
        left = parts[1:4]
        right = parts[6:9]
        result[start] = {"ID":start, "L": left, "R":right}
    return result

def update_node(map, instructions, step, node):
    next = instructions[step % len(instructions)]
    temp = node.get(next)
    node = map.get(temp)
    return node

def part_one_data(data):
    answer = 0
    lines = [x for x in data.split('\n')]
    instructions = lines.pop(0)
    lines.pop(0)
    map = parse_map(lines)
    node = map["AAA"]
    while node.get("ID") != "ZZZ":
        node = update_node(map, instructions, answer, node)
        answer+=1
    return answer

def part_one(file):
    data = open(file).read().strip()
    return part_one_data(data)

def part_two(file):
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    instructions = lines.pop(0)
    lines.pop(0)
    map = parse_map(lines)

    nodes = []
    for node in map.values():
        if node["ID"].endswith("A"):
            nodes.append(node)
    distances = []
    for node in nodes:
        distance = 0
        while not node["ID"].endswith("Z"):
            node = update_node(map, instructions, distance, node)
            distance+=1
        distances.append(distance)
    return math.lcm(*distances)

def benchmark():
    data = open("input-1.txt").read().strip()
    # time_taken = timeit.timeit("part_one_data(data)", globals=globals(), number=100)
    part_one_data(data)
    part_one_data(data)
    part_one_data(data)

    start_time = time.time()
    for _ in range(100):
        part_one_data(data)
    end_time = time.time()
    time_taken = end_time - start_time
    print(f"Time taken to call the function 100 times: {time_taken/100*1000*1000} microseconds")


if __name__ == '__main__':
    expected = 6
    got = part_one('sample-1.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input-1.txt')}")
    benchmark()
    #
    # expected = 6
    # got = part_two('sample-2.txt')
    # match = "match" if expected == got else "don't match"
    # print(f"Sample part 2 {match}, expected {expected}, got {got}")
    # print(f"part 2: {part_two('input-2.txt')}")

