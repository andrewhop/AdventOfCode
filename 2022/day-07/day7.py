class Node:
    def __init__(self, name, parent):
        self.name = name
        # path -> node
        self.children = {}
        self.parent = parent
        # name -> size
        self.items = {}
        self.total_size = 0


def calculate_size(node):
    node_size = 0
    for size in node.items.values():
        node_size += size
    for child in node.children.values():
        node_size += calculate_size(child)
    node.total_size = node_size
    return node_size


def calculate_under(node):
    total_return = 0
    if node.total_size < 100000:
        total_return += node.total_size
    for node in node.children.values():
        total_return += calculate_under(node)
    return total_return


def generate_graph():
    root = Node("/'", None)
    current_node = root
    with open("input-7.txt") as file:
        lines = file.readlines()
        index = 0
        while index < len(lines):
            line = lines[index]
            index += 1
            if "$" in line:
                if "cd" in line:
                    path = line.split(" ")[2].strip()
                    if path == "..":
                        current_node = current_node.parent
                    elif path == "/":
                        current_node = root
                    else:
                        if path in current_node.children.keys():
                            current_node = current_node.children.get(path)
                        else:
                            new_node = Node(path, current_node)
                            current_node.children[path] = new_node
                            current_node = new_node

                elif "ls" in line:
                    while index < len(lines) and "$" not in lines[index]:
                        item = lines[index]
                        index += 1
                        size_or_dir = item.split(" ")[0]
                        name = item.split(" ")[1].strip()
                        if "dir" in size_or_dir:
                            if size_or_dir not in current_node.children.keys():
                                current_node.children[name] = Node(name, current_node)
                        else:
                            size = int(size_or_dir)
                            current_node.items[name] = size
    return root


def puzzle_seven_one():
    root = generate_graph()
    print("done parsing")
    calculate_size(root)
    total = calculate_under(root)
    print(f'7.1 total under 10,000 {total}')


def find_smallest(node, target):
    best_option = node.total_size
    for child in node.children.values():
        if child.total_size > target:
            if child.total_size < best_option:
                best_option = find_smallest(child, target)
    return best_option


def puzzle_seven_two():
    root = generate_graph()
    calculate_size(root)
    total_space = 70000000
    goal = 30000000
    current_free = total_space - root.total_size
    need = goal - current_free
    print(f'7.2 delete {need}')
    smallest = find_smallest(root, need)
    print(f'7.2 smallest for {need} is {smallest}')


if __name__ == '__main__':
    puzzle_seven_one()
    puzzle_seven_two()
