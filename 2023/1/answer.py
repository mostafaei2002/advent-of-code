with open("question", "r") as file:
    question = file.read()

question_lines = question.split("\n")

numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]


def find_matches(line):
    for c in line:
        if c in numbers:
            return c


match_list = []

for line in question_lines:
    current_line = find_matches(line) + find_matches(line[::-1])
    match_list.append(int(current_line))

print(f"Answer is: ", sum(match_list))
