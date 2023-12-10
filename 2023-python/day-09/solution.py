def expand_array(line):
    result = [line]
    while any(item != 0 for item in result[-1]):
        previous_row = result[-1]
        new_line = []
        for i in range(0, len(previous_row) - 1):
            new_line.append(previous_row[i+1] - previous_row[i])
        result.append(new_line)
    return result

def calculate_next_item(history):
    for i, row in enumerate(reversed(history)):
        if all(item == 0 for item in row):
            row.append(0)
        else:
            left = row[-1]
            reversed_i = len(history) - i - 1
            previous = history[reversed_i + 1][-1]
            row.append(left + previous)


def part_one(file):
    answer = 0
    data = open(file).read().strip()
    lines = [batch.split(' ') for batch in data.split('\n')]
    lines = [[int(num) for num in line] for line in lines]
    for line in lines:
        expanded_line = expand_array(line)
        calculate_next_item(expanded_line)
        answer += expanded_line[0][-1]

    return answer

def calculate_previous_item(history):
    for i, row in enumerate(reversed(history)):
        if all(item == 0 for item in row):
            row.append(0)
        else:
            right = row[0]
            reversed_i = len(history) - i - 1
            previous = history[reversed_i + 1][0]
            next = right - previous
            row.insert(0, next)


def part_two(file):
    answer = 0
    data = open(file).read().strip()
    lines = [batch.split(' ') for batch in data.split('\n')]
    lines = [[int(num) for num in line] for line in lines]
    for line in lines:
        expanded_line = expand_array(line)
        calculate_previous_item(expanded_line)
        answer += expanded_line[0][0]

    return answer


if __name__ == '__main__':
    expected = 114
    got = part_one('sample-1.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input-1.txt')}")

    expected = 2
    got = part_two('sample-2.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input-2.txt')}")

