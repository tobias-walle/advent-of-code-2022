package main

import (
	"fmt"
	"tobias-walle/aoc-22/utils"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	total_score, err := get_total_score(lines)
	utils.PanicOnErr(err)

	fmt.Printf("Score: %d", total_score)
}

type Score uint

func get_total_score(lines utils.LineParser) (Score, error) {
	total_score := Score(0)
	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			break
		}

		opponent_shape, err := parse_shape(line[0])
		if err != nil {
			return 0, err
		}

		my_result, err := parse_result(line[2])
		if err != nil {
			return 0, err
		}

		my_shape := get_shape_by_result(opponent_shape, my_result)

		score := my_result.score() + my_shape.score()
		total_score += score
	}
	return total_score, nil
}

type Shape int8

const (
	Rock Shape = iota
	Paper
	Scissors
)

func parse_shape(char byte) (Shape, error) {
	switch char {
	case 'A':
		return Rock, nil
	case 'B':
		return Paper, nil
	case 'C':
		return Scissors, nil
	}
	return 0, fmt.Errorf("shape: cannot be parsed from %c", char)
}

func (shape Shape) score() Score {
	switch shape {
	case Rock:
		return 1
	case Paper:
		return 2
	case Scissors:
		return 3
	}
	return 0
}

type GameResult int8

const (
	Win GameResult = iota
	Draw
	Loose
)

func parse_result(char byte) (GameResult, error) {
	switch char {
	case 'X':
		return Loose, nil
	case 'Y':
		return Draw, nil
	case 'Z':
		return Win, nil
	}
	return 0, fmt.Errorf("shape: cannot be parsed from %c", char)
}

func get_shape_by_result(shape Shape, result GameResult) Shape {
	if result == Draw {
		return shape
	}
	if shape == Rock && result == Win {
		return Paper
	}
	if shape == Rock && result == Loose {
		return Scissors
	}
	if shape == Paper && result == Win {
		return Scissors
	}
	if shape == Paper && result == Loose {
		return Rock
	}
	if shape == Scissors && result == Win {
		return Rock
	}
	if shape == Scissors && result == Loose {
		return Paper
	}
	panic("Unreachable")
}

func (result GameResult) score() Score {
	switch result {
	case Win:
		return 6
	case Draw:
		return 3
	case Loose:
		return 0
	}
	return 0
}
