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
	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			return sum, nil
		}

		// Check which item is in which compartment
		compartment1 := map[byte]bool{}
		compartment2 := map[byte]bool{}
		for i := 0; i < len(line); i++ {
			char := line[i]
			if i < len(line)/2 {
				compartment1[char] = true
			} else {
				compartment2[char] = true
			}
		}

		// Get items that are in both compartments
		both := map[byte]bool{}
		for k := range compartment1 {
			if compartment1[k] && compartment2[k] {
				both[k] = true
			}
		}
		for k := range compartment2 {
			if compartment1[k] && compartment2[k] {
				both[k] = true
			}
		}

		// Calculate priority
		for k := range both {
			sum += getPriority(k)
		}
	}
}

func getPriority(char byte) int {
	// In ASCII, lowercase characters have a lower number than uppercase characters
	isUppercase := char < 'a'
	if isUppercase {
		return int(char - 'A' + 27)
	} else {
		return int(char - 'a' + 1)
	}
}
