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

		my_shape, err := parse_shape(line[2])
		if err != nil {
			return 0, err
		}

		_, my_result := get_result(opponent_shape, my_shape)

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
	case 'A', 'X':
		return Rock, nil
	case 'B', 'Y':
		return Paper, nil
	case 'C', 'Z':
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

func get_result(shape_1 Shape, shape_2 Shape) (GameResult, GameResult) {
	if shape_1 == shape_2 {
		return Draw, Draw
	}
	if shape_1 == Rock && shape_2 == Scissors {
		return Win, Loose
	}
	if shape_1 == Rock && shape_2 == Paper {
		return Loose, Win
	}
	if shape_1 == Paper && shape_2 == Rock {
		return Win, Loose
	}
	if shape_1 == Paper && shape_2 == Scissors {
		return Loose, Win
	}
	if shape_1 == Scissors && shape_2 == Rock {
		return Loose, Win
	}
	if shape_1 == Scissors && shape_2 == Paper {
		return Win, Loose
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
