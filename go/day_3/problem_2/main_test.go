package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
	"tobias-walle/aoc-22/utils"
)

func TestExample(t *testing.T) {
	lines, err := utils.ParseLines("../example.txt")
	utils.PanicOnErr(err)

	result, err := getSumOfPriorities(lines)
	utils.PanicOnErr(err)

	assert.Equal(t, 70, result, "The result should match the example")
}
