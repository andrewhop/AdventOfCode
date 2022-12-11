def puzzle_one_one():
    with open("input-1.txt") as file:
        max_sum = 0
        current_sum = 0
        for line in file:
            if line.isspace():
                max_sum = current_sum if current_sum > max_sum else max_sum
                current_sum = 0
            else:
                current_sum += int(line)
        print(f"Puzzle 1.1 max is: {max_sum}")


def puzzle_one_two():
    with open("input-1.txt") as file:
        all_elves = []
        current_sum = 0
        for line in file:
            if line.isspace():
                all_elves.append(current_sum)
                current_sum = 0
            else:
                current_sum += int(line)
        all_elves.sort(reverse=True)
        print(f"Puzzle 1.2 max is: {all_elves[0] + all_elves[1] + all_elves[2]}")


if __name__ == '__main__':
    puzzle_one_one()
    puzzle_one_two()
