cleaned_games = {}


def read_data():
    with open("../question", "r") as f:
        data = f.read().split("\n")[:-1]

    return data


def clean_data(data):
    for line in data:
        game_id = line.split(" ")[1][:-1]
        game_data = line.split(";")
        game_sets = []

        for game_set in game_data:
            game_set = game_set.split(",")
            current_set = {}
            for subset in game_set:
                cur_subset = subset.split(" ")
                if subset.startswith("Game"):
                    current_set[cur_subset[3]] = cur_subset[2]
                else:
                    current_set[cur_subset[2]] = cur_subset[1]
            game_sets.append(current_set)

        cleaned_games[game_id] = game_sets


def check_possibility(game):
    condition = {"red": 12, "green": 13, "blue": 14}
    for game_set in game:
        for k, v in game_set.items():
            if condition[k] < int(v):
                return False
    return True


def main():
    data = read_data()
    clean_data(data)

    answer = 0
    for game_id, game in cleaned_games.items():
        if check_possibility(game):
            answer += int(game_id)

    print("Answer: ", answer)


if __name__ == "__main__":
    main()
