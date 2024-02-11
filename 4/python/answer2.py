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


def check_row_wrapper(cleaned_row_index, cleaned_data):

    def check_row(cleaned_row_index, cleaned_data, total_cards):
        # print("On card", cleaned_row_index + 1)
        elf_numbers = (
            cleaned_data[cleaned_row_index][0]
            if len(cleaned_data) > cleaned_row_index
            else None
        )
        winning_numbers = (
            cleaned_data[cleaned_row_index][1]
            if len(cleaned_data) > cleaned_row_index
            else None
        )
        num_winning = 0

        if elf_numbers is None or winning_numbers is None:
            return total_cards

        for n in elf_numbers:
            if n in winning_numbers:
                num_winning += 1
        # print("Num matches", num_winning)

        total_cards += num_winning
        for i in range(cleaned_row_index + 1, cleaned_row_index + num_winning + 1):
            total_cards = check_row(i, cleaned_data, total_cards)
        return total_cards

    total_cards = check_row(cleaned_row_index, cleaned_data, 1)
    return total_cards


def get_answer(cleaned_data):
    answer = 0
    for i, _ in enumerate(cleaned_data):
        answer += check_row_wrapper(i, cleaned_data)
    # answer += check_row_wrapper(0, cleaned_data)
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
        with open("../example2") as f:
            data = f.read().splitlines()

        cleaned_data = clean_data(data)
        del data
        answer = get_answer(cleaned_data)
        self.assertEqual(answer, 30)
