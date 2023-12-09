def part_one(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    for line in lines:
        first = -1
        last = -1
        for char in line:
            if 48 <= ord(char) <= 57:
                if first == -1:
                    first = int(char)
                last = int(char)
        answer += first * 10 + last
    return answer

mapping = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

def part_two(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    for line in lines:
        first = -1
        last = -1
        for i in range(0, len(line)):
            char = line[i]
            if 48 <= ord(char) <= 57:
                if first == -1:
                    first = int(char)
                last = int(char)
            else:
                for num, candidate in enumerate(mapping):
                    if line[i:].startswith(candidate):
                        if first == -1:
                            first = num
                        last = num
        answer += first * 10 + last
    return answer


if __name__ == '__main__':
    expected = 142
    got = part_one('sample-1.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input-1.txt')}")

    expected = 281
    got = part_two('sample-2.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input-2.txt')}")

