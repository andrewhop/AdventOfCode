def part_one(file):
    max = {"red": 12, "green": 13, "blue": 14}

    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    for line in lines:
        game = int(line.split(":")[0].split(" ")[1])
        rounds = line.split(":")[1].split(";")
        valid = True
        for round in rounds:
            for move in round.split(','):
                move = move.strip()
                amount = int(move.split(" ")[0])
                color = move.split(" ")[1].removesuffix(',')
                if amount > max.get(color):
                    valid = False
                    break
        if valid:
            answer += game
    return answer


def part_two(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    for line in lines:
        rounds = line.split(":")[1].split(";")
        min = {"red": 0, "green": 0, "blue": 0}
        for round in rounds:
            for move in round.split(','):
                move = move.strip()
                amount = int(move.split(" ")[0])
                color = move.split(" ")[1].removesuffix(',')
                min[color] = max(min[color], amount)
        answer += min["red"] * min["green"] * min["blue"]

    return answer


if __name__ == '__main__':
    expected = 8
    got = part_one('sample-1.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input-1.txt')}")

    expected = 2286
    got = part_two('sample-2.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input-2.txt')}")

