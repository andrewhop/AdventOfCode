# A for Rock
# B for Paper
# C for Scissors

# X for Rock
# Y for Paper
# Z for Scissors

def winner(opponent, me):
    if opponent == "A":
        if me == "X":
            return 1 + 3
        elif me == "Y":
            return 2 + 6
        elif me == "Z":
            return 3 + 0
    elif opponent == "B":
        if me == "X":
            return 1 + 0
        elif me == "Y":
            return 2 + 3
        elif me == "Z":
            return 3 + 6
    elif opponent == "C":
        if me == "X":
            return 1 + 6
        elif me == "Y":
            return 2 + 0
        elif me == "Z":
            return 3 + 3


def puzzle_two_one():
    with open("input-2.txt") as file:
        score = 0
        for line in file:
            parts = line.split(' ')
            opponent = parts[0].strip()
            me = parts[1].strip()
            score += winner(opponent, me)
        print(f'Puzzle 2.1: {score}')


# A for Rock
# B for Paper
# C for Scissors
def winner2(opponent, me):
    if opponent == "A":
        if me == "X":
            return 3 + 0
        elif me == "Y":
            return 1 + 3
        elif me == "Z":
            return 2 + 6
    elif opponent == "B":  # Paper
        if me == "X":
            return 1 + 0
        elif me == "Y":
            return 2 + 3
        elif me == "Z":
            return 3 + 6
    elif opponent == "C":  # Scissors
        if me == "X":
            return 2 + 0
        elif me == "Y":
            return 3 + 3
        elif me == "Z":
            return 1 + 6


def puzzle_two_two():
    with open("input-2.txt") as file:
        score = 0
        for line in file:
            parts = line.split(' ')
            opponent = parts[0].strip()
            me = parts[1].strip()
            score += winner2(opponent, me)
        print(f'Puzzle 2.2: {score}')


if __name__ == '__main__':
    puzzle_two_one()
    puzzle_two_two()
