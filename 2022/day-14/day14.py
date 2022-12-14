AIR = "."
ROCK = "#"
SAND = "o"
SHIFT = 0
SETTLED = 1
OFF_GRID = 0


def input_to_grid(lines):
    grid = []
    biggest_column = 0
    for line in lines:
        points = line.split(" -> ")
        for start, end in zip(points, points[1:]):
            start_column, start_row = [int(x) for x in start.split(',')]
            end_column, end_row = [int(x) for x in end.split(',')]
            start_column -= SHIFT
            end_column -= SHIFT
            biggest_column = max(biggest_column, start_column, end_column)
            if start_row != end_row:
                start_row, end_row = [start_row, end_row] if start_row < end_row else [end_row, start_row]

                for row in range(start_row, end_row + 1):
                    while row >= len(grid):
                        grid.append([])
                    while start_column >= len(grid[row]):
                        grid[row].append(AIR)
                    grid[row][start_column] = ROCK
            if start_column != end_column:
                start_column, end_column = [start_column, end_column] if start_column < end_column else [end_column, start_column]
                for column in range(start_column, end_column + 1):
                    while start_row >= len(grid):
                        grid.append([])
                    while column >= len(grid[start_row]):
                        grid[start_row].append(AIR)
                    grid[start_row][column] = ROCK
    biggest_column += 500
    grid.append([])
    grid.append([])
    # Fill grid
    for row in grid:
        while len(row) <= biggest_column:
            row.append(AIR)

    return grid


class Position:
    def __init__(self, row, column):
        self.row = row
        self.column = column


def simulate_sand(grid):
    sand = Position(0, 500 - SHIFT)
    while True:
        # Fall straight down if possible
        if sand.row == len(grid) - 1:
            return OFF_GRID
        elif grid[sand.row + 1][sand.column] == AIR:
            sand.row += 1
        elif grid[sand.row + 1][sand.column - 1] == AIR:# and grid[sand.row][sand.column - 1] == AIR:
            # sand is below, fall left first
            sand.row += 1
            sand.column -= 1
        elif grid[sand.row + 1][sand.column + 1] == AIR:# and grid[sand.row][sand.column + 1] == AIR:
            # sand is below, fall right
            sand.row += 1
            sand.column += 1
        else:
            # settle
            grid[sand.row][sand.column] = SAND
            return SETTLED


def part_one(file):
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    grid = input_to_grid(lines)
    sand_particles = 0
    while simulate_sand(grid) == SETTLED:
        sand_particles += 1
    return sand_particles

def part_two(file):
    data = open(file).read().strip()
    lines = [x for x in data.split('\n')]
    grid = input_to_grid(lines)
    grid[-1] = [ROCK for i in range(len(grid[0]))]
    sand_particles = 0
    while grid[0][500 - SHIFT] != SAND:
        sand_particles += 1
        simulate_sand(grid)
    return sand_particles


if __name__ == '__main__':
    expected = 24
    got = part_one('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 1 {match}, expected {expected}, got {got}")
    print(f"part 1: {part_one('input.txt')}")
    expected = 93

    got = part_two('sample.txt')
    match = "match" if expected == got else "don't match"
    print(f"Sample part 2 {match}, expected {expected}, got {got}")
    print(f"part 2: {part_two('input.txt')}")