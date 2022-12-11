def puzzle_four_one():
    contained = 0
    with open("input-4.txt") as file:
        for line in file:
            left = line.split(',')[0]
            right = line.split(',')[1].strip()
            left_start = int(left.split('-')[0])
            left_end = int(left.split('-')[1])
            right_start = int(right.split('-')[0])
            right_end = int(right.split('-')[1])
            if left_start <= right_start and left_end >= right_end:
                contained += 1
            elif right_start <= left_start and right_end >= left_end:
                contained += 1
    print(f"Puzzle 4.1 max is: {contained}")


def puzzle_four_two():
    contained = 0
    with open("input-4.txt") as file:
        for line in file:
            left = line.split(',')[0]
            right = line.split(',')[1].strip()
            left_start = int(left.split('-')[0])
            left_end = int(left.split('-')[1])
            right_start = int(right.split('-')[0])
            right_end = int(right.split('-')[1])
            if left_start <= right_start and right_start <= left_end:
                contained += 1
            elif right_start <= left_start and left_start <= right_end:
                contained += 1
            elif left_start <= right_end and right_end <= left_end:
                contained += 1
            elif right_start <= left_end and left_end <= right_end:
                contained += 1
    print(f"Puzzle 4.2 max is: {contained}")


if __name__ == '__main__':
    puzzle_four_one()
    puzzle_four_two()
