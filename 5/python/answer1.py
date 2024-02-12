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
        print(row)
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
        print(row)
        destination_start = row["destination_start"]
        source_start = row["source_start"]
        range_length = row["range"]

        if source >= source_start and source <= (source_start + range_length):
            destination = (source - source_start) + destination_start

    return destination


def get_seed_location(seed, maps):
    cur_val = seed

    for _, mapping in maps.items():
        cur_val = apply_mapping(cur_val, mapping)

    location = cur_val
    return location


def get_answer(seeds, maps):
    locations = [get_seed_location(seed, maps) for seed in seeds]
    # print(seeds, "seeds")
    # print(locations, "locations")
    return min(locations)


def main():
    data = read_data("../question")
    (seeds, maps) = clean_data(data)
    del data
    print(seeds)
    print(maps)
    answer = get_answer(seeds, maps)
    print("answer is:", answer)


if __name__ == "__main__":
    main()


class AnswerTest(unittest.TestCase):

    def test_answer(self):
        data = read_data("../example")
        (seeds, maps) = clean_data(data)
        del data
        print(seeds)
        print(maps)
        answer = get_answer(seeds, maps)
        print("answer is:", answer)
        self.assertEqual(answer, 35)
