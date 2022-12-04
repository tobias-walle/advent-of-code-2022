package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
	"tobias-walle/aoc-22/utils"
)

func TestExample(t *testing.T) {
	lines, err := utils.ParseLines("../example.txt")
	utils.PanicOnErr(err)

	result, err := getNumberOfOverlappingPairs(lines)
	utils.PanicOnErr(err)

	assert.Equal(t, 4, result, "The result should match the example")
}
