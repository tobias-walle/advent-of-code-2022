package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type elf struct {
	number   int
	calories int
}

func main() {
	input_file_path := os.Args[1]
	if input_file_path == "" {
		panic("Please provide the path of the input file as the first argument")
	}

	fmt.Println("Read", input_file_path)
	next_line := read_file_by_line(input_file_path)

	var max elf
	current := elf{number: 1, calories: 0}
	for {
		line, eof := next_line()
		if line != "" {
			// Add calories to current elf
			int_line, err := strconv.Atoi(line)
			check(err)
			current.calories += int_line
			continue
		}

		// Check if calories is in the top 3
		if current.calories > max.calories {
			// Save the new best result
			max = current
		}

		// Next Elf
		current = elf{number: current.number + 1, calories: 0}
		if eof {
			break
		}
	}

	fmt.Printf("Elf %d has the most calories: %d\n", max.number, max.calories)
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
