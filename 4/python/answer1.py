import unittest


def clean_row(row):
    row = row.split(":")
    row = row[1].split("|")
    elf_numbers = row[0].split()
    winning_numbers = row[1].split()
    elf_numbers = [int(n) for n in elf_numbers]
    winning_numbers = [int(n) for n in winning_numbers]

    return [elf_numbers, winning_numbers]


def clean_data(data):
    cleaned_data = []

    for row in data:
        cleaned_row = clean_row(row)
        cleaned_data.append(cleaned_row)
        print(cleaned_row)

    return cleaned_data


def get_row_score(cleaned_row):
    elf_numbers = cleaned_row[0]
    winning_numbers = cleaned_row[1]
    row_score = 0

    for n in elf_numbers:
        if n in winning_numbers and row_score == 0:
            row_score = 1
        elif n in winning_numbers:
            row_score *= 2

    return row_score


def get_answer(cleaned_data):
    answer = 0
    for row in cleaned_data:
        answer += get_row_score(row)
    return answer


def main():

    with open("../question") as f:
        data = f.read().splitlines()

    cleaned_data = clean_data(data)
    del data

    answer = get_answer(cleaned_data)

    print(f"Anwer is: {answer}")


if __name__ == "__main__":
    main()


class AnswerTests(unittest.TestCase):

    def test_example_answer(self):
        with open("../example") as f:
            data = f.read().splitlines()

        cleaned_data = clean_data(data)
        del data
        answer = get_answer(cleaned_data)
        self.assertEqual(answer, 13)
