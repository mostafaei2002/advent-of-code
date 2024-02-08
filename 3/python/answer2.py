lines = None


def read_data():
    with open("../question", "r") as f:
        data = f.read().split("\n")[:-1]

    return data


def get_char(i, j):
    try:
        return lines[i][j]
    except IndexError:
        return None


def get_full_num(start_i, start_j):
    cur_num = lines[start_i][start_j]

    i = start_i
    j = start_j + 1
    try:
        while lines[i][j].isnumeric() and j >= 0 and j < len(lines[i]):
            cur_num = cur_num + lines[i][j]
            j += 1
    except Exception as e:
        print(e)

    try:
        j = start_j - 1
        while lines[i][j].isnumeric() and j >= 0 and j < len(lines[i]):
            cur_num = lines[i][j] + cur_num
            j -= 1
    except Exception as e:
        print(e)

    return cur_num


def check_star(i, j):
    adjacent_positions = {
        "top_left": (i - 1, j - 1),
        "top": (i - 1, j),
        "top_right": (i - 1, j + 1),
        "bottom_left": (i + 1, j - 1),
        "bottom": (i + 1, j),
        "bottom_right": (i + 1, j + 1),
        "left": (i, j - 1),
        "right": (i, j + 1),
    }
    adjacent_chars = {}
    for k, v in adjacent_positions.items():
        (row, col) = v
        if row >= 0 and col >= 0 and col < len(lines[i]):
            adjacent_chars[k] = get_char(row, col)
        else:
            adjacent_chars[k] = None

    num1_pos = None
    num2_pos = None
    find = True
    found_key = ""
    for n, (k, v) in enumerate(adjacent_chars.items()):

        if v is not None and not v.isnumeric():
            find = True
        elif found_key[:2] != k[:2]:  # New Line
            find = True

        if v is not None and v.isnumeric() and find and num1_pos:
            num2_pos = k
            break
        elif v is not None and v.isnumeric() and find:
            num1_pos = k
            found_key = k
            find = False

    if num1_pos and num2_pos:
        num1 = get_full_num(*adjacent_positions[num1_pos])
        num2 = get_full_num(*adjacent_positions[num2_pos])

        if num1 == num2:
            print(f"edge case on like {i+1}")

        print(num1, num2)
        return int(num1) * int(num2)

    # print("Edge case mb?", num1_pos, num2_pos)
    return 0


def main():
    answer = 0
    global lines
    lines = read_data()

    for i in range(len(lines)):
        print(f"line {i+1}")
        for j in range(len(lines[i])):
            if lines[i][j] == "*":
                answer += check_star(i, j)

    print(answer)


if __name__ == "__main__":
    main()
