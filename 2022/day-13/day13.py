import functools
import json

CORRECT_ORDER = -1
INCORRECT_ORDER = 1
UNKNOWN = 0


def compare_values(left, right):
    if type(left) == list and type(right) == list:
        return compare_lists(left, right)
    elif type(left) == int and type(right) == int:
        left_int = int(left)
        right_int = int(right)
        if left_int < right_int:
            return CORRECT_ORDER
        elif left_int > right_int:
            return INCORRECT_ORDER
        else:
            return UNKNOWN
    elif type(left) == list and type(right) != list:
        right = [right]
        return compare_lists(left, right)
    elif type(left) != list and type(right) == list:
        left = [left]
        return compare_lists(left, right)

    raise Exception("Bad state")


def compare_lists(left, right):
    for i in range(len(left)):
        if i > len(right) - 1:
            return INCORRECT_ORDER
        if type(left[i]) == list and type(right[i]) == list:
            answer = compare_lists(left[i], right[i])
            if answer != UNKNOWN:
                return answer
            else:
                continue
        else:
            answer = compare_values(left[i], right[i])
            if answer != UNKNOWN:
                return answer
            else:
                continue
    if len(left) < len(right):
        return CORRECT_ORDER
    else:
        return UNKNOWN


def part_one(file):
    answer = 0
    data = open(file).read().strip()
    batches = [batch.split('\n') for batch in data.split('\n\n')]
    for index, batch in enumerate(batches):
        left, right = batch
        left_json = json.loads(left)
        right_json = json.loads(right)
        correct_order = compare_lists(left_json, right_json)
        answer = answer + index + 1 if correct_order == CORRECT_ORDER else answer
    return answer


def part_two(file):
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    clean = []
    for line in lines:
        if line != "":
            clean.append(json.loads(line))
    clean.append([[2]])
    clean.append([[6]])
    clean = sorted(clean, key=functools.cmp_to_key(compare_lists))
    start = 0
    end = 0
    for index, line in enumerate(clean):
        if line == [[2]]:
            start = index + 1
        elif line == [[6]]:
            end = index + 1
    return start * end


if __name__ == '__main__':
    expected = 13
    got = part_one('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input.txt')}")

    expected = 140
    got = part_two('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input.txt')}")
