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
	debug := os.Getenv("DEBUG") == "true"
	visited := map[Position]bool{}
	headPosition := Position{}
	tail := [9]Position{}
	visited[tail[0]] = true
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
			positionToCompareWith := headPosition
			positionIndex := 0
			for shouldTailMove(positionToCompareWith, tail[positionIndex]) {
				tail[positionIndex] = moveTailPiece(tail[positionIndex], positionToCompareWith)
				positionToCompareWith = tail[positionIndex]
				positionIndex++

				if positionIndex >= len(tail) {
					break
				}
			}
			visited[tail[len(tail)-1]] = true

			if debug {
				fmt.Print("\033[H\033[2J") // Clear
				fmt.Println(line, "/", i+1)
				render(headPosition, tail, visited)
				waitForInput()
			}
		}
	}

	numberOfTailPositions := 0
	for _, visited := range visited {
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

func render(head Position, tail [9]Position, tailPositions map[Position]bool) {
	rowsMin := 0
	colsMin := 0
	rowsMax := 4
	colsMax := 5

	rowsMin = Min(rowsMin, head.y)
	rowsMax = Max(rowsMax, head.y)
	colsMin = Min(colsMin, head.x)
	colsMax = Max(colsMax, head.x)

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
				continue
			}

			found := false
			for i, t := range tail {
				if t == p {
					print(i + 1)
					found = true
					break
				}
			}
			if found {
				continue
			}

			if tailPositions[p] {
				print("#")
				continue
			}

			print(".")
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
