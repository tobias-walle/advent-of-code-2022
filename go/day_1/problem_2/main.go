package main

import (
	"fmt"
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

	// Top 3 max calories, starting with the higher value
	var top [3]elf
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
		for i, max := range top {
			if current.calories > max.calories {
				// Shift the best result to the right
				if i+1 < len(top) {
					top[i+1] = top[i]
				}
				// Save the new best result
				top[i] = current
				break
			}
		}

		// Next Elf
		current = elf{number: current.number + 1, calories: 0}
		if eof {
			break
		}
	}

	sum_calories := 0
	for _, elf := range top {
		sum_calories += elf.calories
	}

	fmt.Printf("Elf %d has the most calories: %d\n", top[0].number, top[0].calories)
	fmt.Printf("Elf %d has the second most calories: %d\n", top[1].number, top[1].calories)
	fmt.Printf("Elf %d has the third most calories: %d\n", top[2].number, top[2].calories)
	fmt.Printf("Sum: %d\n", sum_calories)
}
