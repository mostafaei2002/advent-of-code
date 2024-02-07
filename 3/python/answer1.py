def read_data():
    with open("../question", "r") as f:
        data = f.read().split("\n")[:-1]

    return data


def get_char(list, index):
    try:
        return list[0][index]
    except IndexError:
        return "."


def main():
    lines = read_data()
    answer = 0
    symbols = ["*", "#", "$", "+", "=", "/", "@", "%", "&", "^", "!", "-"]

    for i in range(len(lines)):
        prev_ln = lines[i - 1 : i]
        cur_ln = lines[i : i + 1]
        next_ln = lines[i + 1 : i + 2]
        cur_num = ""
        add_num = False
        cur_line = ""
        for j in range(len(cur_ln[0])):
            prev_ln_char = get_char(prev_ln, j)
            cur__ln_char = get_char(cur_ln, j)
            next_ln_char = get_char(next_ln, j)

            if (prev_ln_char in symbols) or (next_ln_char in symbols):
                add_num = True
            if cur__ln_char in symbols:
                add_num = True

            if (cur__ln_char in symbols) and (cur_num != ""):
                target_num = int(cur_num)
                cur_line += f"{target_num}, "
                answer += target_num

                add_num = True
                cur_num = ""

            elif (
                cur__ln_char == "."
                and add_num is True
                and cur_num != ""
                and (prev_ln_char in symbols or next_ln_char in symbols)
            ):
                target_num = int(cur_num)
                cur_line += f"{target_num}, "
                answer += target_num

                add_num = True
                cur_num = ""

            elif cur__ln_char == "." and add_num is True and cur_num != "":
                target_num = int(cur_num)
                cur_line += f"{target_num}, "
                answer += target_num

                add_num = False
                cur_num = ""

            elif cur__ln_char == "." and (
                (prev_ln_char in symbols) or (next_ln_char in symbols)
            ):
                add_num = True
                cur_num = ""
            elif cur__ln_char == ".":
                add_num = False
                cur_num = ""
            elif cur__ln_char.isnumeric():
                cur_num += cur__ln_char

            # Handle end of line
            if (j == len(cur_ln[0]) - 1) and add_num is True:
                answer += int(cur_num)
                cur_line += f"{target_num}, "

        print(cur_line)

    print(answer)


if __name__ == "__main__":
    main()
