package main

import (
	"tobias-walle/aoc-22/utils"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	sum, err := getSumOfPriorities(lines)
	utils.PanicOnErr(err)
	println("Sum of priorities:", sum)
}

func getSumOfPriorities(lines utils.LineParser) (int, error) {
	sum := 0
	groupItems := map[byte]bool{}
	index := 0
	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			return sum, nil
		}

		indexInGroup := index % 3

		if indexInGroup == 0 {
			// For the first rucksack in a group, set all items to true
			for i := 0; i < len(line); i++ {
				char := line[i]
				groupItems[char] = true
			}
		} else {
			// For all other rucksacks remove the items that are not included
			items := map[byte]bool{}
			for i := 0; i < len(line); i++ {
				char := line[i]
				items[char] = true
			}
			for k := range groupItems {
				groupItems[k] = groupItems[k] && items[k]
			}
		}

		// Calculate priority
		if indexInGroup == 2 {
			for k, included := range groupItems {
				if included {
					sum += getPriority(k)
				}
			}
			groupItems = map[byte]bool{}
		}

		index++
	}
}

func getPriority(char byte) int {
	// In ASCII, lowercase characters have a lower number than uppercase characters
	is_uppercase := char < 'a'
	if is_uppercase {
		return int(char - 'A' + 27)
	} else {
		return int(char - 'a' + 1)
	}
}
