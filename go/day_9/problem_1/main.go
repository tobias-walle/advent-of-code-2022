package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"tobias-walle/aoc-22/utils"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	result, err := getResult(lines)
	utils.PanicOnErr(err)
	println(result)
}

func getResult(lines utils.LineParser) (int, error) {
	debug := true
	tailPositions := map[Position]bool{}
	headPosition := Position{}
	tailPosition := Position{}
	tailPositions[tailPosition] = true
	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			break
		}

		var direction Direction
		items := strings.Split(line, " ")
		direction = items[0]
		movesString := items[1]
		moves, err := strconv.Atoi(string(movesString))
		if err != nil {
			return 0, err
		}

		for i := 0; i < moves; i++ {
			headPosition = headPosition.move(direction)
			if shouldTailMove(headPosition, tailPosition) {
				tailPosition = moveTailPiece(tailPosition, headPosition)
				tailPositions[tailPosition] = true
			}
			if debug {
				fmt.Print("\033[H\033[2J") // Clear
				fmt.Println(line, "/", i+1)
				render(headPosition, tailPosition, tailPositions)
				waitForInput()
			}
		}
	}

	numberOfTailPositions := 0
	for _, visited := range tailPositions {
		if visited {
			numberOfTailPositions += 1
		}
	}

	return numberOfTailPositions, nil
}

func shouldTailMove(p1 Position, p2 Position) bool {
	if Abs(p1.x-p2.x) == 2 {
		return true
	}
	if Abs(p1.y-p2.y) == 2 {
		return true
	}
	return false
}

func moveTailPiece(tailPiece Position, head Position) Position {
	if tailPiece.y < head.y {
		tailPiece.y += 1
	}
	if tailPiece.y > head.y {
		tailPiece.y -= 1
	}
	if tailPiece.x < head.x {
		tailPiece.x += 1
	}
	if tailPiece.x > head.x {
		tailPiece.x -= 1
	}
	return tailPiece
}

type Direction = string

const (
	UP    Direction = "U"
	RIGHT           = "R"
	DOWN            = "D"
	LEFT            = "L"
)

type Position struct {
	x int
	y int
}

func (pos Position) move(direction Direction) Position {
	if direction == UP {
		pos.y += 1
	} else if direction == RIGHT {
		pos.x += 1
	} else if direction == DOWN {
		pos.y -= 1
	} else if direction == LEFT {
		pos.x -= 1
	}
	return pos
}

func render(head Position, tail Position, tailPositions map[Position]bool) {
	rowsMin := 0
	colsMin := 0
	rowsMax := 4
	colsMax := 5

	rowsMin = Min(Min(rowsMin, head.y), tail.y)
	rowsMax = Max(Max(rowsMax, head.y), tail.y)
	colsMin = Min(Min(colsMin, head.x), tail.x)
	colsMax = Max(Max(colsMax, head.x), tail.x)

	for p := range tailPositions {
		rowsMin = Min(rowsMin, p.y)
		rowsMax = Max(rowsMax, p.y)
		colsMin = Min(colsMin, p.x)
		colsMax = Max(colsMax, p.x)
	}

	for y := rowsMax; y >= rowsMin; y-- {
		for x := colsMin; x <= colsMax; x++ {
			p := Position{x: x, y: y}
			if head == p {
				print("H")
			} else if tail == p {
				print("T")
			} else if tailPositions[p] {
				print("#")
			} else {
				print(".")
			}
		}
		println()
	}
}

func Min(a int, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}

func Max(a int, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

func Abs(a int) int {
	if a < 0 {
		return -a
	} else {
		return a
	}
}

func waitForInput() {
	input := bufio.NewScanner(os.Stdin)
	input.Scan()
}
