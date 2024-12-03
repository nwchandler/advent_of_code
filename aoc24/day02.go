package aoc24

import (
	"bufio"
	"io"
	"math"
	"strconv"
	"strings"
)

func Day02(input io.Reader) (Result, error) {
	buf := bufio.NewScanner(input)
	reports := [][]int{}
	for buf.Scan() {
		tmp := []int{}
		for _, number := range strings.Fields(buf.Text()) {
			num, err := strconv.Atoi(number)
			if err != nil {
				return Result{}, err
			}
			tmp = append(tmp, num)
		}
		reports = append(reports, tmp)
	}

	var isSafe func([]int, bool) bool
	isSafe = func(report []int, dampen bool) bool {
		if len(report) == 1 {
			// The case of a report with a single level wasn't called out in the
			// problem, so I'm making an assumption here that if there is only
			// one level, it is safe. It also obviates some additional index
			// checks below.
			return true
		}

		increasing := report[1] > report[0]
		valid := true
		i := 1
		for i < len(report) {
			this := report[i]
			prev := report[i-1]
			diff := math.Abs(float64(this) - float64(prev))
			if diff > 3 || diff < 1 {
				valid = false
				break
			}
			if increasing && this <= prev {
				valid = false
				break
			}
			if !increasing && this >= prev {
				valid = false
				break
			}
			i++
		}
		if !valid && dampen {
			// Because we break on the first detected failure above,
			// we can try removing this element or earlier ones.
			// In general, it seems unuseful to attempt replacing
			// any more than three (i-2, i-1, i) elements. For the
			// size of records in the input, this doesn't save us
			// much, but for extremely long reports, keeping this
			// linear time could add up.
			for j := i; j >= 0 && j >= i-2; j-- {
				tmp := []int{}
				tmp = append(tmp, report[:j]...)
				tmp = append(tmp, report[j+1:]...)
				valid = isSafe(tmp, false)
				if valid {
					return true
				}
			}
		}
		return valid
	}

	part1 := func() string {
		safe := 0
		for _, report := range reports {
			if isSafe(report, false) {
				safe++
			}
		}
		return strconv.Itoa(safe)
	}

	part2 := func() string {
		safe := 0
		for _, report := range reports {
			if isSafe(report, true) {
				safe++
			}
		}
		return strconv.Itoa(safe)
	}

	return Result{
		Part1: part1(),
		Part2: part2(),
	}, nil
}
