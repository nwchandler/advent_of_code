package aoc24

import (
	"bufio"
	"io"
	"slices"
	"strconv"
)

func Day04(input io.Reader) (Result, error) {
	buf := bufio.NewScanner(input)
	graph := [][]byte{}
	for buf.Scan() {
		// pad with extra zeros to simplify bounds checking later
		tmp := []byte{0, 0, 0}
		tmp = append(tmp, buf.Bytes()...)
		tmp = append(tmp, []byte{0, 0, 0}...)
		graph = append(graph, tmp)
	}
	lineLength := len(graph[0])

	for range 3 {
		// pad with extra zeros to simplify bounds checking later
		tmp := make([]byte, lineLength)
		graph = append(graph, tmp[:])
		graph = append([][]byte{tmp[:]}, graph...)
	}

	neighbors := map[string][]int{
		"north":     {-1, 0},
		"northeast": {-1, 1},
		"east":      {0, 1},
		"southeast": {1, 1},
		"south":     {1, 0},
		"southwest": {1, -1},
		"west":      {0, -1},
		"northwest": {-1, -1},
	}

	part1 := func() string {
		var search func(int, int, []int, []byte) []byte
		search = func(row int, col int, direction []int, path []byte) []byte {
			if len(path) == 4 {
				return path
			}
			path = append(path, graph[row][col])
			neighborRow := row + direction[0]
			neighborCol := col + direction[1]
			return search(neighborRow, neighborCol, direction, path)
		}

		target := []byte{'X', 'M', 'A', 'S'}
		sum := 0
		for i, row := range graph {
			for j, cell := range row {
				if cell == 'X' {
					// this could be the start of a new XMAS!
					for _, neighbor := range neighbors {
						path := []byte{'X'}
						neighborRow := i + neighbor[0]
						neighborCol := j + neighbor[1]
						path = search(neighborRow, neighborCol, neighbor, path)
						if slices.Equal(target, path) {
							sum++
						}
					}
				}
			}
		}
		return strconv.Itoa(sum)
	}

	part2 := func() string {
		sum := 0
		for i, row := range graph {
			for j, cell := range row {
				// let's just look for the center
				if cell == 'A' {
					ne := graph[i+neighbors["northeast"][0]][j+neighbors["northeast"][1]]
					se := graph[i+neighbors["southeast"][0]][j+neighbors["southeast"][1]]
					sw := graph[i+neighbors["southwest"][0]][j+neighbors["southwest"][1]]
					nw := graph[i+neighbors["northwest"][0]][j+neighbors["northwest"][1]]

					if ne == 'M' && sw == 'S' ||
						ne == 'S' && sw == 'M' {

						if nw == 'M' && se == 'S' ||
							nw == 'S' && se == 'M' {
							sum++
						}
					}
				}
			}
		}
		return strconv.Itoa(sum)
	}

	return Result{
		Part1: part1(),
		Part2: part2(),
	}, nil
}
