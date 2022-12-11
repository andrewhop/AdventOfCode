from math import floor


def puzzle_ten_one():
    cycle = 0
    X = 1
    signal_sum = 0
    target = [20, 60, 100, 140, 180, 220]
    with open("input-10.txt") as file:
        for line in file:
            if "noop" in line:
                cycle += 1
                if cycle in target:
                    signal_sum += X * cycle
            else:
                for i in range(2):
                    cycle += 1
                    if cycle in target:
                        signal_sum += X * cycle
                amount = line.split(" ")[1].strip()
                X += int(amount)
    print(f"took {cycle}, final value {X}, signal_sum {signal_sum}")


def puzzle_ten_two():
    crt = [[' ' for i in range(40)] for i in range(6)]
    cycle = 0
    X = 1
    with open("input-10.txt") as file:
        for line in file:
            if "noop" in line:
                if abs(X - (cycle % 40)) <= 1:
                    row = floor(cycle / 40)
                    column = cycle % 40
                    crt[row][column] = "#"
                cycle += 1
            else:
                for i in range(2):
                    if abs(X - (cycle % 40)) <= 1:
                        row = floor(cycle / 40)
                        column = cycle % 40
                        crt[row][column] = "#"
                    cycle += 1

                amount = line.split(" ")[1].strip()
                X += int(amount)
    for row in crt:
        for column in row:
            print(column, end='')
        print('')


if __name__ == '__main__':
    puzzle_ten_one()
    puzzle_ten_two()
