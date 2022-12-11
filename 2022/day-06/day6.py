def puzzle_six_one():
    with open("input-6.txt") as file:
        for line in file:
            start = 0
            slidding_window = []
            for char in line:
                slidding_window.append(char)
                start += 1
                if len(slidding_window) > 4:
                    slidding_window.pop(0)
                if len(set(slidding_window)) >= 4:
                    print(f'{line} starts at {start}')
                    break


def puzzle_six_two():
    with open("input-6.txt") as file:
        for line in file:
            start = 0
            slidding_window = []
            for char in line:
                slidding_window.append(char)
                start += 1
                if len(slidding_window) > 14:
                    slidding_window.pop(0)
                if len(set(slidding_window)) >= 14:
                    print(f'{line} starts at {start}')
                    break


if __name__ == '__main__':
    puzzle_six_one()
    puzzle_six_two()
