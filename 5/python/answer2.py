import concurrent.futures
import unittest


def read_data(path):
    with open(path, "r") as f:
        data = f.read().splitlines()
    return data


def clean_data(data):
    seeds = []
    maps = {}

    i = 0
    map_name = None
    while i < len(data):
        row = data[i]
        # print(row)
        if map_name and row:
            cur_range = row.split()
            cur_range = [int(n) for n in cur_range]

            maps[map_name].append(
                {
                    "destination_start": cur_range[0],
                    "source_start": cur_range[1],
                    "range": cur_range[2],
                }
            )

        elif row.startswith("seeds:"):
            seeds.extend(row.split()[1:])

        elif row.endswith("map:"):
            map_name = row
            maps[map_name] = []

        else:
            map_name = None

        i += 1

    seeds = [int(s) for s in seeds]

    return (seeds, maps)


def apply_mapping(source, mapping):
    destination = source

    for row in mapping:
        # print(row)
        destination_start = row["destination_start"]
        source_start = row["source_start"]
        range_length = row["range"]

        if source >= source_start and source <= (source_start + range_length):
            destination = (source - source_start) + destination_start

    return destination


def get_seed_location(seed, maps):
    cur_val = seed
    if seed % 1000000 == 0:
        print(seed)

    for _, mapping in maps.items():
        cur_val = apply_mapping(cur_val, mapping)

    location = cur_val
    return location


def check_batch(cur, end):
    global maps
    min_location = float("inf")
    with concurrent.futures.ThreadPoolExecutor() as executor:
        tasks = [
            executor.submit(get_seed_location, seed, maps) for seed in range(cur, end)
        ]
        for future in concurrent.futures.as_completed(tasks):
            if future.result():
                min_location = min(min_location, future.result())
    return min_location


def check_range(start_range, end_range):
    cur = start_range
    min_location = float("inf")
    print("Starting a new task")
    while cur < end_range:
        end = cur + 1000000
        if end > end_range:
            end = end_range
        min_location = min(min_location, check_batch(cur, end))
        cur += 1000000

    return min_location


def get_answer(seed_range_map, maps):
    min_location = float("inf")

    for item in seed_range_map:
        start_range = item["range_start"]
        end_range = item["range_start"] + item["range_length"]
        min_location = min(min_location, check_range(start_range, end_range))

    return min_location


def get_all_ranges(seed_row):
    seed_range = []
    for i in range(0, len(seed_row), 2):
        seed_range.append({"range_start": seed_row[i], "range_length": seed_row[i + 1]})

    return seed_range


def main():
    global maps
    data = read_data("../question")
    (seed_row, maps) = clean_data(data)
    del data
    seed_range_map = get_all_ranges(seed_row)
    answer = get_answer(seed_range_map, maps)
    print("answer is:", answer)


if __name__ == "__main__":
    main()


class AnswerTest(unittest.TestCase):

    def test_answer(self):
        data = read_data("../example")
        (seed_row, maps) = clean_data(data)
        del data
        seed_range_map = get_all_ranges(seed_row)
        answer = get_answer(seed_range_map, maps)
        print("answer is:", answer)
        self.assertEqual(answer, 47)
