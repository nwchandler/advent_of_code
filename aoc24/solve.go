package aoc24

import (
	"fmt"
	"io"
)

// Result contains the results of both parts of a day's problem.
type Result struct {
	Part1 string
	Part2 string
}

// Solve attempts to solve the day's problem, given the provided input.
func Solve(day int, input io.Reader) (Result, error) {
	switch day {
	case 1:
		return Day01(input)
	case 2:
		return Day02(input)
	case 3:
		return Day03(input)
	case 4:
		return Day04(input)
	case 5:
		return Day05(input)
	}
	return Result{}, fmt.Errorf("no solvers found for day %d", day)
}
