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

	visible := map[Tree]bool{}

	for row := range grid {
		for col := range grid[row] {
			if isTreeVisible(grid, row, col) {
				visible[getTree(grid, row, col)] = true
			}
		}
	}

	numberOfVisibleTrees := 0
	for range visible {
		numberOfVisibleTrees++
	}

	return numberOfVisibleTrees, nil
}

func isTreeVisible(grid Grid, row int, col int) bool {
	tree := getTree(grid, row, col)
	maxRow := len(grid) - 1
	maxCol := len(grid[col]) - 1

	// Right
	visible := true
	for r := row + 1; r <= maxRow; r++ {
		if grid[r][col] >= tree.value {
			visible = false
			break
		}
	}
	if visible {
		return true
	}

	// Left
	visible = true
	for r := row - 1; r >= 0; r-- {
		if grid[r][col] >= tree.value {
			visible = false
			break
		}
	}
	if visible {
		return true
	}

	// Bottom
	visible = true
	for c := col + 1; c <= maxCol; c++ {
		if grid[row][c] >= tree.value {
			visible = false
			break
		}
	}
	if visible {
		return true
	}

	// Top
	visible = true
	for c := col - 1; c >= 0; c-- {
		if grid[row][c] >= tree.value {
			visible = false
			break
		}
	}
	if visible {
		return true
	}

	return false
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
