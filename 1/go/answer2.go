package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func find_matches(line string) (n1 string, n2 string) {
	valuemap := map[string]string{
		"one":   "1",
		"two":   "2",
		"three": "3",
		"four":  "4",
		"five":  "5",
		"six":   "6",
		"seven": "7",
		"eight": "8",
		"nine":  "9",
		"1":     "1",
		"2":     "2",
		"3":     "3",
		"4":     "4",
		"5":     "5",
		"6":     "6",
		"7":     "7",
		"8":     "8",
		"9":     "9",
	}

	var m1 string = ""
	var m2 string = ""

	for i := 0; i < len(line); i++ {
		for j := i + 1; j < i+6; j++ {
			if j > len(line) {
				continue
			}
			cur_slice := line[i:j]

			value, ok := valuemap[cur_slice]
			if ok && m1 == "" {
				m1 = value
			}
			if ok {
				m2 = value
			}
		}
	}

	return m1, m2
}

func main() {
	data, err := os.ReadFile("../question")

	if err != nil {
		fmt.Println("Failed to read file")
		return
	}

	data_lines := strings.Split(string(data), "\n")

	var sum int64 = 0

	for _, line := range data_lines {
		n1, n2 := find_matches(line)

		cur_match, err := strconv.ParseInt((n1 + n2), 10, 64)
		if err != nil {
			fmt.Println("Failed to parse int")
			return
		}

		sum += cur_match
	}

	fmt.Println(sum)
}
