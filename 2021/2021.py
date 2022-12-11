# This is a sample Python script.

# Press Shift+F10 to execute it or replace it with your code.
# Press Double Shift to search everywhere for classes, files, tool windows, actions, and settings.

def puzzle_one_one():
    increases = 0
    with open("input-1.txt") as file:
        previous = int(file.readline())
        for line in file:
            next = int(line)
            if next > previous:
                increases += 1
            previous = next
    print(f'Puzzle 1.1: {increases}')


def puzzle_one_two():
    increases = 0

    def sub_sum(measurements, start):
        first = int(measurements[start])
        second = int(measurements[start + 1])
        third = int(measurements[start + 2])
        return first + second + third

    with open("input-1.txt") as file:
        measurements = file.readlines()
        previous = sub_sum(measurements, 0)
        for i in range(1, len(measurements) - 2):
            next = sub_sum(measurements, i)
            if next > previous:
                increases += 1
            previous = next
    print(f'Puzzle 1.2: {increases}')

def puzzle_two_one():
    horizontal = 0
    depth = 0
    with open("input-2.txt") as file:
        for line in file:
            direction, amount = line.split(' ')
            if direction == "forward":
                horizontal += int(amount)
            elif direction == "up":
                depth -= int(amount)
            elif direction == "down":
                depth += int(amount)
            else:
                print(f'Unknown direction {direction}')
                exit()
    print(f'Went {horizontal} forward, {depth} total depth, for puzzle answer {horizontal * depth}')

# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    puzzle_two_one()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
