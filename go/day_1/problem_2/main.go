package main

import (
	"bufio"
	"fmt"
	"github.com/thoas/go-funk"
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

	type result struct {
		elf      int
		calories int
	}

	// Top 3 max calories, starting with the higher value
	var results [3]result
	for elf, calories_of_elf := range calories {
		for i, max := range results {
			if calories_of_elf > max.calories {
				// Shift the best result to the right
				if i+1 < len(results) {
					results[i+1] = results[i]
				}
				// Save the new best result
				results[i] = result{
					elf:      elf,
					calories: calories_of_elf,
				}
				break
			}
		}
	}

	var result_calories [3]int
	for i, result := range results {
		result_calories[i] = result.calories
	}

	fmt.Printf("Elf %d has the most calories: %d\n", results[0].elf+1, results[0].calories)
	fmt.Printf("Elf %d has the second most calories: %d\n", results[1].elf+1, results[1].calories)
	fmt.Printf("Elf %d has the third most calories: %d\n", results[2].elf+1, results[2].calories)
	fmt.Printf("Sum: %d\n", funk.SumInt(result_calories[:]))
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
