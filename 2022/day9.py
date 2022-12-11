class Position:
    def __init__(self):
        self.x = 0
        self.y = 0


def puzzle_nine_one():
    head = Position()
    tail = Position()
    tail_visited = [(0, 0)]
    with open("input-9.txt") as file:
        for line in file:
            direction = line.split(" ")[0]
            amount = int(line.split(" ")[1].strip())
            for i in range(amount):
                if direction == "U":
                    head.y += 1
                elif direction == "D":
                    head.y -= 1
                elif direction == "L":
                    head.x -= 1
                elif direction == "R":
                    head.x += 1

                should_move = abs(tail.x - head.x) > 1 or abs(tail.y - head.y) > 1

                if should_move:
                    if tail.x < head.x:
                        tail.x += 1
                    elif tail.x > head.x:
                        tail.x -= 1
                    if tail.y < head.y:
                        tail.y += 1
                    elif tail.y > head.y:
                        tail.y -= 1
                    tail_visited.append((tail.x, tail.y))
    print(f'tail visited {len(set(tail_visited))}')


def puzzle_nine_two():
    knots = [Position() for i in range(10)]
    tail_visited = [(0, 0)]
    with open("input-9.txt") as file:
        for line in file:
            direction = line.split(" ")[0]
            amount = int(line.split(" ")[1].strip())
            for i in range(amount):
                for i in range(len(knots)):
                    if i == 0:
                        if direction == "U":
                            knots[0].y += 1
                        elif direction == "D":
                            knots[0].y -= 1
                        elif direction == "L":
                            knots[0].x -= 1
                        elif direction == "R":
                            knots[0].x += 1
                    else:
                        head = knots[i - 1]
                        tail = knots[i]
                        should_move = abs(tail.x - head.x) > 1 or abs(tail.y - head.y) > 1

                        if should_move:
                            if tail.x < head.x:
                                tail.x += 1
                            elif tail.x > head.x:
                                tail.x -= 1
                            if tail.y < head.y:
                                tail.y += 1
                            elif tail.y > head.y:
                                tail.y -= 1
                            if i == 9:
                                tail_visited.append((tail.x, tail.y))
    print(f'tail visited {len(set(tail_visited))}')


if __name__ == '__main__':
    puzzle_nine_one()
    puzzle_nine_two()
