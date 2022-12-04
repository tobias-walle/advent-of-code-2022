package main

import (
	"fmt"
	"tobias-walle/aoc-22/utils"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	totalScore, err := getTotalScore(lines)
	utils.PanicOnErr(err)

	fmt.Printf("Score: %d", totalScore)
}

type Score uint

func getTotalScore(lines utils.LineParser) (Score, error) {
	totalScore := Score(0)
	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			break
		}

		opponentShape, err := parseShape(line[0])
		if err != nil {
			return 0, err
		}

		myShape, err := parseShape(line[2])
		if err != nil {
			return 0, err
		}

		_, myResult := getResult(opponentShape, myShape)

		score := myResult.score() + myShape.score()
		totalScore += score
	}
	return totalScore, nil
}

type Shape int8

const (
	Rock Shape = iota
	Paper
	Scissors
)

func parseShape(char byte) (Shape, error) {
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

func getResult(shape1 Shape, shape2 Shape) (GameResult, GameResult) {
	if shape1 == shape2 {
		return Draw, Draw
	}
	if shape1 == Rock && shape2 == Scissors {
		return Win, Loose
	}
	if shape1 == Rock && shape2 == Paper {
		return Loose, Win
	}
	if shape1 == Paper && shape2 == Rock {
		return Win, Loose
	}
	if shape1 == Paper && shape2 == Scissors {
		return Loose, Win
	}
	if shape1 == Scissors && shape2 == Rock {
		return Loose, Win
	}
	if shape1 == Scissors && shape2 == Paper {
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
