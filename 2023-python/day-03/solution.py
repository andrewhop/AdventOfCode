import re

symbols = ['@', '#', '$', '%', '&', '*', '-', '+', '=', '/']
pattern = r'\d+'
search_directions = [-1, 0, 1]

def is_next_to_symbol(lines, start_row, start_column, length):
    for i in range(start_column, start_column + length):
        for column_offset in search_directions:
            column = i + column_offset
            if column < 0 or column >= len(lines[start_row]):
                continue
            for row_offset in search_directions:
                row = start_row + row_offset
                if row < 0 or row >= len(lines):
                    continue
                if lines[row][column] in symbols:
                    return True
    return False


def part_one(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    for line_number, line in enumerate(lines):
        matches = re.finditer(pattern, line)
        for match in matches:
            start = match.start()
            length = len(match.group())

            if is_next_to_symbol(lines, line_number, start, length):
                number = int(match.group())
                answer += number

    return answer

def is_gear(lines, start_row, start_column):
    numbers = set()
    for column_offset in search_directions:
        column = start_column + column_offset
        if column < 0 or column >= len(lines[start_row]):
            continue
        for row_offset in search_directions:
            row = start_row + row_offset
            if row < 0 or row >= len(lines):
                continue
            if lines[row][column].isdigit():
                num_start = column
                num_end = column
                while num_start > 0 and lines[row][num_start - 1].isdigit():
                    num_start -= 1
                while num_end < len(lines[row]) - 1 and lines[row][num_end + 1].isdigit():
                    num_end += 1
                numbers.add(int(lines[row][num_start:num_end + 1]))
    if len(numbers) == 2:
        return list(numbers)
    else:
        return [0,0]





def part_two(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    for row, line in enumerate(lines):
        for column, charecter in enumerate(line):
            if lines[row][column] == '*':
                left, right = is_gear(lines, row, column)
                answer += left * right

    return answer


if __name__ == '__main__':
    expected = 4361
    got = part_one('sample-1.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input-1.txt')}")
    expected = 467835
    got = part_two('sample-2.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input-2.txt')}")

