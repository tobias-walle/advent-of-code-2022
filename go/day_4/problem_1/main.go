package main

import (
	"fmt"
	"strconv"
	"strings"
	"tobias-walle/aoc-22/utils"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	sum, err := getNumberOfFullyContainingPairs(lines)
	utils.PanicOnErr(err)
	println("Number of overlapping pairs:", sum)
}

func getNumberOfFullyContainingPairs(lines utils.LineParser) (int, error) {
	number_fully_containing_pairs := 0

	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			break
		}

		ranges, err := parseRanges(line)
		if err != nil {
			return 0, err
		}

		if ranges[0].isContaining(ranges[1]) || ranges[1].isContaining(ranges[0]) {
			number_fully_containing_pairs++
		}

	}

	return number_fully_containing_pairs, nil
}

func parseRanges(input string) ([]Range, error) {
	ranges := []Range{}
	splits := strings.Split(input, ",")
	for _, split := range splits {
		parsed, err := parseRange(split)
		if err != nil {
			return ranges, err
		}
		ranges = append(ranges, parsed)
	}
	return ranges, nil
}

func parseRange(input string) (Range, error) {
	splits := strings.Split(input, "-")
	if len(splits) != 2 {
		return Range{}, fmt.Errorf("Expected only two parts for range, got %d", len(splits))
	}
	start, err := strconv.Atoi(splits[0])
	if err != nil {
		return Range{}, err
	}
	end, err := strconv.Atoi(splits[1])
	if err != nil {
		return Range{}, err
	}
	return Range{start: start, end: end}, nil
}

type Range struct {
	start int
	end   int
}

func (r Range) isContaining(other Range) bool {
	return r.start <= other.start && r.end >= other.end
}
