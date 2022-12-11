def puzzle_eight_one():
    grid = []
    visible_trees = 0
    with open("input-8.txt") as file:
        for line in file:
            grid.append([])
            for char in line.strip():
                grid[-1].append(int(char))

        for row_index in range(len(grid)):
            row = grid[row_index]
            for column_index in range(len(row)):
                tree = row[column_index]
                if row_index == 0 or column_index == 0 or row_index == len(grid) - 1 or column_index == len(row) - 1:
                    visible_trees += 1
                    continue
                # going up
                up_visible = True
                for test_row in range(row_index - 1, -1, -1):
                    if grid[test_row][column_index] >= tree:
                        up_visible = False
                        break
                down_visible = True
                for test_row in range(row_index + 1, len(grid), 1):
                    if grid[test_row][column_index] >= tree:
                        down_visible = False
                        break
                left_visible = True
                for test_column in range(column_index - 1, -1, -1):
                    if grid[row_index][test_column] >= tree:
                        left_visible = False
                        break
                right_visible = True
                for test_column in range(column_index + 1, len(row), 1):
                    if grid[row_index][test_column] >= tree:
                        right_visible = False
                        break
                if up_visible or down_visible or left_visible or right_visible:
                    visible_trees += 1

    print(f"Puzzle 8.1 trees is: {visible_trees}")


def puzzle_eight_two():
    grid = []
    max_score = 0
    with open("input-8.txt") as file:
        for line in file:
            grid.append([])
            for char in line.strip():
                grid[-1].append(int(char))

        for row_index in range(len(grid)):
            row = grid[row_index]
            for column_index in range(len(row)):
                tree = row[column_index]
                up_trees = 0
                for test_row in range(row_index - 1, -1, -1):
                    up_trees += 1
                    if grid[test_row][column_index] >= tree:
                        break
                down_trees = 0
                for test_row in range(row_index + 1, len(grid), 1):
                    down_trees += 1
                    if grid[test_row][column_index] >= tree:
                        break
                left_trees = 0
                for test_column in range(column_index - 1, -1, -1):
                    left_trees += 1
                    if grid[row_index][test_column] >= tree:
                        break
                right_trees = 0
                for test_column in range(column_index + 1, len(row), 1):
                    right_trees += 1
                    if grid[row_index][test_column] >= tree:
                        break
                tree_score = left_trees * right_trees * up_trees * down_trees
                max_score = tree_score if tree_score > max_score else max_score

    print(f"Puzzle 8.2 max is: {max_score}")


if __name__ == '__main__':
    puzzle_eight_one()
    puzzle_eight_two()
