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


def get_all_ranges(seed_row):
    seed_range = []
    for i in range(0, len(seed_row), 2):
        seed_range.append({"range_start": seed_row[i], "range_length": seed_row[i + 1]})

    return seed_range


def check_seed_exists(seed, seeds_range):
    for item in seeds_range:
        start = item["range_start"]
        end = item["range_start"] + item["range_length"]
        if seed >= start and seed < end:
            return True
    return False


def apply_reverse_mapping(source, mapping):
    destination = source

    for v in list(mapping.values())[::-1]:
        # print(v)
        source_start = v[0]["destination_start"]  # reverse
        destination_start = v[0]["source_start"]
        range_length = v[0]["range"]

        if source >= source_start and source <= (source_start + range_length):
            destination = (source - source_start) + destination_start

    return destination


def handle_location_map(location_map, seeds_range, maps):
    # print(location_map)
    locations_start = location_map["destination_start"]
    locations_end = location_map["destination_start"] + location_map["range"]
    result = None

    for location in range(locations_start, locations_end):
        seed = apply_reverse_mapping(location, maps)
        found = check_seed_exists(seed, seeds_range)

        if found:
            result = location

    return result


def get_answer(location_maps_sorted, seeds_range, maps):
    for location_map in location_maps_sorted:
        result = handle_location_map(location_map, seeds_range, maps)
        if result:
            break

    return result


def main():
    data = read_data("../question")
    (seed_row, maps) = clean_data(data)
    del data
    location_maps = None
    for k, v in maps.items():
        if k.endswith("-location map:"):
            location_maps = v
    location_maps_sorted = sorted(location_maps, key=lambda i: i["destination_start"])
    del location_maps

    seeds_range = get_all_ranges(seed_row)

    answer = get_answer(location_maps_sorted, seeds_range, maps)
    print("answer is:", answer)


if __name__ == "__main__":
    main()


class AnswerTest(unittest.TestCase):

    def test_answer(self):
        data = read_data("../example")
        (seed_row, maps) = clean_data(data)
        del data
        location_maps = None
        for k, v in maps.items():
            if k.endswith("-location map:"):
                location_maps = v
        location_maps_sorted = sorted(
            location_maps, key=lambda i: i["destination_start"]
        )
        del location_maps

        seeds_range = get_all_ranges(seed_row)

        answer = get_answer(location_maps_sorted, seeds_range, maps)
        # print("answer is:", answer)
        self.assertEqual(answer, 47)
