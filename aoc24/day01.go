package aoc24

import (
	"bufio"
	"fmt"
	"io"
	"slices"
	"strconv"
	"strings"
)

func Day01(input io.Reader) (Result, error) {
	buf := bufio.NewScanner(input)
	lists := [2][]int{}
	for buf.Scan() {
		// We're playing a little fast and loose here with decoding, but
		// I think that's fair given the low stakes of AOC :)
		numbers := strings.Fields(buf.Text())
		for i := range 2 {
			num, err := strconv.Atoi(numbers[i])
			if err != nil {
				return Result{}, err
			}
			lists[i] = append(lists[i], num)
		}
	}
	if len(lists[0]) != len(lists[1]) {
		return Result{}, fmt.Errorf("something went wrong; lists are different sizes [%d vs [%d]", len(lists[0]), len(lists[1]))
	}

	slices.Sort(lists[0])
	slices.Sort(lists[1])

	part1 := func() string {
		distance := 0
		for i := range len(lists[0]) {
			dist := lists[0][i] - lists[1][i]
			if dist < 0 {
				distance += -dist
			} else {
				distance += dist
			}
		}
		return strconv.Itoa(distance)
	}

	part2 := func() string {
		distance := 0
		l2Map := map[int]int{}
		for i := range lists[1] {
			l2Map[lists[1][i]]++
		}
		for i := range lists[0] {
			distance += lists[0][i] * l2Map[lists[0][i]]
		}
		return strconv.Itoa(distance)
	}

	return Result{
		Part1: part1(),
		Part2: part2(),
	}, nil
}
