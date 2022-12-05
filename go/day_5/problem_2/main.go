package main

import (
	"fmt"
	"strconv"
	"strings"
	"tobias-walle/aoc-22/utils"
	"unicode"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	result, err := getCratesAtTopOfStacks(lines)
	utils.PanicOnErr(err)
	println(result)
}

func getCratesAtTopOfStacks(lines utils.LineParser) (string, error) {
	stacks := Stacks{}
	for {
		line, done, err := lines.Next()
		if err != nil {
			return "", err
		}
		if done {
			break
		}

		if strings.HasPrefix(line, "m") {
			// Parse Moves

			// Debug Log
			for _, s := range stacks {
				for _, c := range s {
					fmt.Printf("%c", c)
				}
				fmt.Println()
			}
			fmt.Println(line)

			// Extract infos
			words := strings.Split(line, " ")
			moveN, err := strconv.Atoi(words[1])
			if err != nil {
				return "", err
			}
			moveFrom, err := strconv.Atoi(words[3])
			if err != nil {
				return "", err
			}
			moveTo, err := strconv.Atoi(words[5])
			if err != nil {
				return "", err
			}

			// Move crates
			stackFrom := stacks[moveFrom-1]
			stackTo := stacks[moveTo-1]
			var crates []Crate
			stackFrom, crates = popN(stackFrom, moveN)
			stackTo = append(stackTo, crates...)
			stacks[moveFrom-1] = stackFrom
			stacks[moveTo-1] = stackTo
		} else {
			// Parse Crates
			for i, crate := range line {
				if i%4 != 1 || crate == ' ' {
					continue
				}
				if unicode.IsDigit(crate) {
					break
				}
				stackIndex := i / 4
				stack := get(stacks, stackIndex)
				stacks = ensureCapacity(stacks, stackIndex+1)
				stack = prepend(stack, crate)
				stacks[stackIndex] = stack
			}
		}
	}

	// Get highest crates
	result := ""
	for _, stack := range stacks {
		if len(stack) > 0 {
			result = fmt.Sprintf("%s%c", result, stack[len(stack)-1])
		}
	}

	return result, nil
}

type Crate = rune
type Stacks = [][]Crate
type Stack = []Crate

func ensureCapacity[T any](list []T, size int) []T {
	if size > len(list) {
		for i := len(list); i < size; i++ {
			var item T
			list = append(list, item)
		}
	}
	return list
}

func prepend[T any](list []T, value T) []T {
	return append([]T{value}, list...)
}

func get[T any](list []T, index int) T {
	var item T
	if index < len(list) {
		item = list[index]
	}
	return item
}

func popN[T any](list []T, n int) (rest []T, popped []T) {
	if len(list) == n {
		return []T{}, list
	}
	return list[:len(list)-n], list[len(list)-n:]
}

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}
