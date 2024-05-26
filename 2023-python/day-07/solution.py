def part_one(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    batches = [batch.split('\n') for batch in data.split('\n\n')]
    for line in lines:
        pass
    for batch in batches:
        first, second, third, forth = batch
    return answer


def part_two(file):
    answer = 0
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    batches = [batch.split('\n') for batch in data.split('\n\n')]
    for line in lines:
        pass
    for batch in batches:
        first, second, third, forth = batch
    return answer


if __name__ == '__main__':
    for i in range(2000):
        print(i, end=",")
    # expected = 0
    # got = part_one('sample-1.txt')
    # match = "match" if expected == got else "don't match"
    # print(f"Sample part 1 {match}, expected {expected}, got {got}")
    # print(f"part 1: {part_one('input-1.txt')}")
    #
    # expected = 0
    # got = part_two('sample-2.txt')
    # match = "match" if expected == got else "don't match"
    # print(f"Sample part 2 {match}, expected {expected}, got {got}")
    # print(f"part 2: {part_two('input-2.txt')}")

