package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	input_file_path := os.Args[1]
	if input_file_path == "" {
		panic("Please provide the path of the input file as the first argument")
	}

	fmt.Println("Read", input_file_path)
	next_line := read_file_by_line(input_file_path)

	calories := []int{0}
	elf := 0
	for {
		line, eof := next_line()
		if eof {
			break
		}

		if line == "" {
			// Next Elf
			elf++
			calories = append(calories, 0)
			continue
		}

		// Add calories to current elf
		line_int, err := strconv.Atoi(line)
		check(err)
		calories[elf] += line_int
	}

	max_elf := 0
	max_calories := calories[0]
	for elf, calories_of_elf := range calories {
		if calories_of_elf > max_calories {
			max_calories = calories_of_elf
			max_elf = elf
		}
	}

	fmt.Printf("Elf %d has the most calories:\n%d", max_elf+1, max_calories)
}

func read_file_by_line(path string) func() (line string, eof bool) {
	input_file, err := os.Open(path)
	check(err)

	scanner := bufio.NewScanner(input_file)
	scanner.Split(bufio.ScanLines)
	return func() (line string, eof bool) {
		if scanner.Scan() {
			return scanner.Text(), false
		}
		defer input_file.Close()
		check(scanner.Err())
		return "", true
	}
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
