package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func find_matches(line string) (n1 string, n2 string) {
	var m1 string = ""
	var m2 string = ""
	for i := 0; i < len(line); i++ {
		matches, err := regexp.MatchString(`[0-9]`, line[i:i+1])

		if err != nil {
			fmt.Println(err)
			return
		}

		if matches {
			if m1 == "" {
				m1 = line[i : i+1]
			}

			m2 = line[i : i+1]
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
