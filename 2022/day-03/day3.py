def puzzle_three_one():
    sum = 0

    with open("input-3.txt") as file:
        for line in file:
            line = line.strip()

            split = int(len(line) / 2)
            firstpart = line[0:split]
            secondpart = line[split:len(line)]
            common = []
            for char in firstpart:
                if char in secondpart:
                    common.append(char)
            for letter in set(common):
                if letter.isupper():
                    priority = ord(letter) - ord('A') + 27
                else:
                    priority = ord(letter) - ord('a') + 1
                sum += priority

    print(sum)


def find_common(line):
    split = int(len(line) / 2)

    firstpart = line[0:split]
    secondpart = line[split:len(line)]
    common = []
    for char in firstpart:
        if char in secondpart:
            common.append(char)

    return set(common)


def priority(char):
    if char.isupper():
        priority = ord(char) - ord('A') + 27
    else:
        priority = ord(char) - ord('a') + 1
    return priority


def puzzle_three_two():
    sum = 0

    with open("input-3.txt") as file:
        alllines = file.readlines()

        for i in range(0, len(alllines) - 2, 3):
            firstchar = set(alllines[i].strip())
            secondchar = set(alllines[i + 1].strip())
            thirdchar = set(alllines[i + 2].strip())
            common = firstchar.intersection(secondchar).intersection(thirdchar)
            for char in common:
                sum += priority(char)
    print(sum)


if __name__ == '__main__':
    puzzle_three_one()
    puzzle_three_two()
