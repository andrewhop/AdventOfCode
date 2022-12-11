def puzzle_five_one():
    buckets = [[]]
    with open("input-5.txt") as file:
        for line in file:
            if '[' in line:
                line = line[:-1]
                chunks = [line[i:i + 4] for i in range(0, len(line), 4)]
                for i in range(len(chunks)):
                    if len(buckets) <= i:
                        buckets.append([])
                    container = chunks[i][1].strip()
                    if container != '':
                        buckets[i].append(container)
            elif "move" in line:
                chunks = line.split(' ')
                amount = int(chunks[1])
                source = int(chunks[3]) - 1
                destination = int(chunks[5]) - 1
                for i in range(amount):
                    to_move = buckets[source].pop(0)
                    buckets[destination].insert(0, to_move)
    for bucket in buckets:
        print(bucket[0], end='')
    print("\n")


def puzzle_five_two():
    buckets = [[]]
    with open("input-5.txt") as file:
        for line in file:
            if '[' in line:
                line = line[:-1]
                chunks = [line[i:i + 4] for i in range(0, len(line), 4)]
                for i in range(len(chunks)):
                    if len(buckets) <= i:
                        buckets.append([])
                    container = chunks[i][1].strip()
                    if container != '':
                        buckets[i].append(container)
            elif "move" in line:
                chunks = line.split(' ')
                amount = int(chunks[1])
                source = int(chunks[3]) - 1
                destination = int(chunks[5]) - 1
                to_move = []
                for i in range(amount):
                    to_move.insert(0, buckets[source].pop(0))
                for i in range(amount):
                    buckets[destination].insert(0, to_move[i])

    for bucket in buckets:
        print(bucket[0], end='')
    print("\n")


if __name__ == '__main__':
    puzzle_five_one()
    puzzle_five_two()
