import time
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


def cleanup(item):
    item = item.strip()
    return int(item)


def puzzle_eleven_one():
    monkeys = []
    with open("input-11.txt") as file:
        lines = file.readlines()
        index = 0
        while index < len(lines):
            # Monkey num
            index += 1
            items = list(map(cleanup, lines[index].split(":")[1].split(",")))
            index += 1
            operation_line = lines[index].split("=")[1].strip()
            index += 1
            test = int(lines[index].split("by")[1].strip())
            index += 1
            true = int(lines[index].split("monkey")[1].strip())
            index += 1
            false = int(lines[index].split("monkey")[1].strip())
            monkeys.append(Monkey(items, operation_line, test, true, false))
            index += 2

    for i in range(20):
        for monkey in monkeys:
            for item in monkey.items:
                monkey.inspected += 1
                old = item
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
    print(f'11.1 answer {throws[0] * throws[1]}')


def puzzle_eleven_two():
    monkeys = []
    common_multiple = 1
    with open("input-11.txt") as file:
        lines = file.readlines()
        index = 0
        while index < len(lines):
            # Monkey num
            index += 1
            items = list(map(cleanup, lines[index].split(":")[1].split(",")))
            index += 1
            operation_line = lines[index].split("=")[1].strip()
            index += 1
            test = int(lines[index].split("by")[1].strip())
            common_multiple *= test
            index += 1
            true = int(lines[index].split("monkey")[1].strip())
            index += 1
            false = int(lines[index].split("monkey")[1].strip())
            monkeys.append(Monkey(items, operation_line, test, true, false))
            index += 2

    for i in range(10000):
        for monkey in monkeys:
            for item in monkey.items:
                monkey.inspected += 1
                old = item
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
    print(f'11.2 answer {throws[0] * throws[1]}')


if __name__ == '__main__':
    puzzle_eleven_one()
    puzzle_eleven_two()
