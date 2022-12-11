from functools import reduce
from math import floor


class Monkey:
    def __init__(self, items, operation, test, true, false):
        # each item is a worry level int
        self.items = items
        self.operation = operation
        # integer
        self.test = test
        self.to_pass = {True: true, False: false}
        self.inspected = 0


def part_one(file):
    monkeys = []
    data = open(file).read().strip()
    batches = [batch.split('\n') for batch in data.split('\n\n')]
    for batch in batches:
        num, items, operation, test, true, false = batch
        items = [int(i) for i in items.split(':')[1].split(',')]
        operation = operation.split("=")[1].strip()
        test = int(test.split()[-1])
        true = int(true.split()[-1])
        false = int(false.split()[-1])
        monkeys.append(Monkey(items, operation, test, true, false))
    for i in range(20):
        for monkey in monkeys:
            for old in monkey.items:
                monkey.inspected += 1
                new_worry = eval(monkey.operation)
                new_worry = floor(new_worry / 3)
                should_throw = new_worry % monkey.test == 0
                new_monkey = monkey.to_pass[should_throw]
                monkeys[new_monkey].items.append(new_worry)
            monkey.items = []
    throws = []
    for monkey in monkeys:
        throws.append(monkey.inspected)
    throws.sort(reverse=True)
    return throws[0] * throws[1]


def part_two(file):
    monkeys = []
    data = open(file).read().strip()
    batches = [batch.split('\n') for batch in data.split('\n\n')]
    for batch in batches:
        num, items, operation, test, true, false = batch
        items = [int(i) for i in items.split(':')[1].split(',')]
        operation = operation.split("=")[1].strip()
        test = int(test.split()[-1])
        true = int(true.split()[-1])
        false = int(false.split()[-1])
        monkeys.append(Monkey(items, operation, test, true, false))
    common_multiple = reduce((lambda x, y: x * y), [val.test for val in monkeys])
    for i in range(10000):
        for monkey in monkeys:
            for old in monkey.items:
                monkey.inspected += 1
                new_worry = eval(monkey.operation)
                new_worry = new_worry % common_multiple
                should_throw = new_worry % monkey.test == 0
                new_monkey = monkey.to_pass[should_throw]
                monkeys[new_monkey].items.append(new_worry)
            monkey.items = []
    throws = []
    for monkey in monkeys:
        throws.append(monkey.inspected)
    throws.sort(reverse=True)
    return throws[0] * throws[1]


if __name__ == '__main__':
    expected = 10605
    got = part_one('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input.txt')}")

    expected = 2713310158
    got = part_two('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input.txt')}")
