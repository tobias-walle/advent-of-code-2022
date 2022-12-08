package main

import (
	"strconv"
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
	grid := Grid{}
	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			break
		}
		lineInt := []int{}
		for _, char := range line {
			charInt, err := strconv.Atoi(string(char))
			if err != nil {
				return 0, err
			}
			lineInt = append(lineInt, charInt)
		}
		grid = append(grid, lineInt)
	}

	maxScore := 0
	for row := range grid {
		for col := range grid[row] {
			score := getScore(grid, row, col)
			if score > maxScore {
				maxScore = score
			}
		}
	}

	return maxScore, nil
}

func getScore(grid Grid, row int, col int) int {
	tree := getTree(grid, row, col)
	maxRow := len(grid) - 1
	maxCol := len(grid[col]) - 1

	right := 0
	for r := row + 1; r <= maxRow; r++ {
		right++
		if grid[r][col] >= tree.value {
			break
		}
	}

	left := 0
	for r := row - 1; r >= 0; r-- {
		left++
		if grid[r][col] >= tree.value {
			break
		}
	}

	bottom := 0
	for c := col + 1; c <= maxCol; c++ {
		bottom++
		if grid[row][c] >= tree.value {
			break
		}
	}

	top := 0
	for c := col - 1; c >= 0; c-- {
		top++
		if grid[row][c] >= tree.value {
			break
		}
	}

	return right * left * top * bottom
}

func getTree(grid Grid, row int, col int) Tree {
	return Tree{row: row, col: col, value: grid[row][col]}
}

type Grid = [][]int

type Tree struct {
	row   int
	col   int
	value int
}
