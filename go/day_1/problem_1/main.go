package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"tobias-walle/aoc-22/utils"
)

type elf struct {
	number   int
	calories int
}

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	var max elf
	current := elf{number: 1, calories: 0}
	for {
		line, eof, err := lines.Next()
		if err != nil {
			utils.PanicOnErr(err)
		}
		if line != "" {
			// Add calories to current elf
			int_line, err := strconv.Atoi(line)
			utils.PanicOnErr(err)
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
	utils.PanicOnErr(err)

	scanner := bufio.NewScanner(input_file)
	scanner.Split(bufio.ScanLines)
	return func() (line string, eof bool) {
		if scanner.Scan() {
			return scanner.Text(), false
		}
		defer input_file.Close()
		utils.PanicOnErr(scanner.Err())
		return "", true
	}
}
