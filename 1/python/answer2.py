with open("../question", "r") as file:
    question = file.read()

question_lines = question.split("\n")


values = {
    "zero": 0,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "0": 0,
    "1": 1,
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
}


def find_matches(line):
    first_match = ""
    last_match = ""
    for i in range(len(line)):
        for j in range(1, 6):
            cur = line[i : i + j]
            if cur in values.keys():
                last_match = values[cur]
                if first_match == "":
                    first_match = values[cur]

    print(first_match, last_match)
    return str(first_match), str(last_match)


answer = 0

for line in question_lines:
    print(line)
    first_match, last_match = find_matches(line)
    answer += int(first_match + last_match)

print(f"Answer is: ", answer)
