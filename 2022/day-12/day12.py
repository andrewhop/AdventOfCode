def can_step(current, target):
    if target == "S":
        return False
    if target == "E":
        target = 'z'
    if current == "S":
        current = 'a'
    current = ord(current)
    target = ord(target)
    return target - current <= 1


directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]


def get_value(data, position):
    return data[int(position[1])][int(position[0])]


def part_one(file):
    answer = 0
    start = ()
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    # length of row
    max_x = len(lines[0])
    # length of columns
    max_y = len(lines)

    for num, line in enumerate(lines):
        if "S" in line:
            start = (line.find("S"), num)

    paths = [[start]]
    visited = {start: [start]}

    # only go to a node if the path to get there is shorter than before
    while len(paths) > 0:
        current_path = paths.pop(0)
        current_position = current_path[-1]
        current_value = get_value(lines, current_position)
        if current_value == "E":
            return len(current_path) - 1

        for direction in directions:
            spot_position = tuple(map(sum, zip(current_position, direction)))
            if spot_position[0] < 0 or spot_position[0] >= max_x:
                continue
            if spot_position[1] < 0 or spot_position[1] >= max_y:
                continue
            spot_value = get_value(lines, spot_position)
            if can_step(current_value, spot_value):
                new_path = list(current_path)
                new_path.append(spot_position)
                current_shortest = visited.get(spot_position, None)
                if current_shortest is None or len(new_path) < len(current_shortest):
                    paths.append(new_path)
                    visited[spot_position] = new_path
    return answer


def can_step2(current, target):
    if current == "E":
        current = 'z'
    if current == "S":
        current = 'a'
    if target == "S":
        target = 'a'
    if target == "E":
        target = "z"
    current = ord(current)
    target = ord(target)
    return current - target <= 1


def part_two(file):
    answer = 0
    start = ()
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    max_x = len(lines[0])
    max_y = len(lines)

    for num, line in enumerate(lines):
        if "E" in line:
            start = (line.find("E"), num)

    paths = [[start]]
    visited = {start: [start]}

    while len(paths) > 0:
        current_path = paths.pop(0)
        current_position = current_path[-1]
        current_value = get_value(lines, current_position)
        if current_value == "a":
            return len(current_path) - 1

        for direction in directions:
            spot_position = tuple(map(sum, zip(current_position, direction)))
            if spot_position[0] < 0 or spot_position[0] >= max_x:
                continue
            if spot_position[1] < 0 or spot_position[1] >= max_y:
                continue
            spot_value = get_value(lines, spot_position)
            if can_step2(current_value, spot_value):
                new_path = list(current_path)
                new_path.append(spot_position)
                current_shortest = visited.get(spot_position, None)
                if current_shortest is None or len(new_path) < len(current_shortest):
                    paths.append(new_path)
                    visited[spot_position] = new_path
    return answer


if __name__ == '__main__':
    expected = 31
    got = part_one('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input.txt')}")

    expected = 29
    got = part_two('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input.txt')}")
