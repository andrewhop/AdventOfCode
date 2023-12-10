import math

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
    temp = node[next]
    node = map[temp]
    return node

def part_one(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    instructions = lines.pop(0)
    lines.pop(0)
    map = parse_map(lines)
    node = map["AAA"]
    while node["ID"] != "ZZZ":
        node = update_node(map, instructions, answer, node)
        answer+=1
    return answer


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



if __name__ == '__main__':
    expected = 6
    got = part_one('sample-1.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input-1.txt')}")

    expected = 6
    got = part_two('sample-2.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input-2.txt')}")

