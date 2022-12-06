package main

import (
	"fmt"
	"tobias-walle/aoc-22/utils"

	"golang.org/x/exp/utf8string"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	result, err := getStartOfPacketMarker(lines)
	utils.PanicOnErr(err)
	println(result)
}

func getStartOfPacketMarker(lines utils.LineParser) (int, error) {
	line, _, err := lines.Next()
	if err != nil {
		return 0, err
	}

	input := utf8string.NewString(line)

	for i := 0; i < input.RuneCount(); i++ {
		if i < 3 {
			continue
		}
		last := map[rune]bool{}
		count := 0
		for j := 0; j < 4; j++ {
			char := input.At(i - j)
			if last[char] {
				break
			}
			last[char] = true
			count++
		}
		if count == 4 {
			return i + 1, nil
		}
	}

	return 0, fmt.Errorf("No marker found")
}
