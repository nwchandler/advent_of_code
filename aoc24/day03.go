package aoc24

import (
	"bufio"
	"io"
	"regexp"
	"strconv"
)

func Day03(input io.Reader) (Result, error) {
	buf := bufio.NewScanner(input)

	// the input is only one line, so just scan once
	buf.Scan()
	line := buf.Text()

	part1 := func() (string, error) {
		mulRE, err := regexp.Compile(`mul\(\d{1,3},\d{1,3}\)`)
		if err != nil {
			return "", err
		}
		operations := mulRE.FindAllString(line, -1)
		numRE, err := regexp.Compile(`\d{1,3}`)
		if err != nil {
			return "", err
		}

		sum := 0
		for _, operation := range operations {
			nums := numRE.FindAllString(operation, -1)
			n1, err := strconv.Atoi(nums[0])
			if err != nil {
				return "", err
			}
			n2, err := strconv.Atoi(nums[1])
			if err != nil {
				return "", err
			}
			sum += n1 * n2
		}
		return strconv.Itoa(sum), nil
	}

	part2 := func() (string, error) {
		mulRE, err := regexp.Compile(`mul\(\d{1,3},\d{1,3}\)`)
		if err != nil {
			return "", err
		}

		doRE, err := regexp.Compile(`do\(\)`)
		if err != nil {
			return "", err
		}
		enables := doRE.FindAllStringIndex(line, -1)

		dontRE, err := regexp.Compile(`don't\(\)`)
		if err != nil {
			return "", err
		}
		disables := dontRE.FindAllStringIndex(line, -1)

		intervals := [][]int{{0, len(line)}}
		enabled := true
		for len(enables) > 0 && len(disables) > 0 {
			if enables[0][0] < disables[0][0] {
				if !enabled {
					enabled = true
					intervals = append(intervals, []int{enables[0][0], len(line)})
				}
				enables = enables[1:]
			} else {
				if enabled {
					enabled = false
					intervals[len(intervals)-1][1] = disables[0][0]
				}
				disables = disables[1:]
			}
		}
		// if we have any remaining enables or disables, let's update
		// our intervals accordingly
		if len(enables) > 0 && !enabled {
			intervals = append(intervals, []int{enables[0][0], len(line)})
		} else if len(disables) > 0 && enabled {
			intervals[len(intervals)-1][1] = disables[0][0]
		}

		numRE, err := regexp.Compile(`\d{1,3}`)
		if err != nil {
			return "", err
		}

		sum := 0
		for _, interval := range intervals {
			start := interval[0]
			stop := interval[1]
			operations := mulRE.FindAllString(line[start:stop], -1)

			for _, operation := range operations {
				nums := numRE.FindAllString(operation, -1)
				num1, err := strconv.Atoi(nums[0])
				if err != nil {
					return "", err
				}
				num2, err := strconv.Atoi(nums[1])
				if err != nil {
					return "", err
				}
				sum += num1 * num2
			}
		}
		return strconv.Itoa(sum), nil
	}

	p1, err := part1()
	if err != nil {
		return Result{}, err
	}

	p2, err := part2()
	if err != nil {
		return Result{}, err
	}

	return Result{
		Part1: p1,
		Part2: p2,
	}, nil
}
