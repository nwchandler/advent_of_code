package aoc24

import (
	"bufio"
	"io"
	"strconv"
	"strings"
)

func Day05(input io.Reader) (Result, error) {
	buf := bufio.NewScanner(input)

	// orderings will store the adjacencies between nodes
	orderings := map[int]map[int]bool{}

	// The input has a blank line between the ordering rules and
	// the pages to produce, so once we hit an empty line, we
	// have finished building our orderings.
	for buf.Scan() && buf.Text() != "" {
		numbers := strings.Split(buf.Text(), "|")
		predecessor, err := strconv.Atoi(numbers[0])
		if err != nil {
			return Result{}, err
		}

		successor, err := strconv.Atoi(numbers[1])
		if err != nil {
			return Result{}, err
		}

		if _, ok := orderings[predecessor]; !ok {
			orderings[predecessor] = map[int]bool{}
		}

		orderings[predecessor][successor] = true
	}

	updates := [][]int{}
	for buf.Scan() {
		tmp := []int{}
		numbers := strings.Split(buf.Text(), ",")
		for _, num := range numbers {
			page, err := strconv.Atoi(num)
			if err != nil {
				return Result{}, err
			}
			tmp = append(tmp, page)
		}
		updates = append(updates, tmp)
	}

	unordered := [][]int{}

	part1 := func() string {
		sum := 0
	OUTER:
		for _, update := range updates {
			for lastPage := len(update) - 1; lastPage > 0; lastPage-- {
				for priorPage := lastPage - 1; priorPage >= 0; priorPage-- {
					if orderings[update[lastPage]][update[priorPage]] {
						// this helps save a little work in part 2
						unordered = append(unordered, update)
						continue OUTER
					}
				}
			}
			middlePage := len(update) / 2
			sum += update[middlePage]
		}

		return strconv.Itoa(sum)
	}

	part2 := func() string {
		sum := 0
		for _, update := range unordered {
			lastPage := len(update) - 1
		OUTER:
			for lastPage > 0 {
				for priorPage := lastPage - 1; priorPage >= 0; priorPage-- {
					if orderings[update[lastPage]][update[priorPage]] {
						update[lastPage], update[priorPage] = update[priorPage], update[lastPage]
						lastPage = len(update) - 1
						continue OUTER
					}
				}
				lastPage--
			}
			middlePage := len(update) / 2
			sum += update[middlePage]
		}

		return strconv.Itoa(sum)
	}

	return Result{
		Part1: part1(),
		Part2: part2(),
	}, nil
}
