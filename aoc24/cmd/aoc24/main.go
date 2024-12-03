package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/nwchandler/advent_of_code/aoc24"
)

func main() {
	day := 0
	flag.IntVar(&day, "day", 0, "provide the number of the day")
	flag.Parse()

	if day == 0 {
		log.Fatal("must provide a day number using the `day` flag")
	}

	inputFilename := fmt.Sprintf("day%d.txt", day)
	input, err := os.Open(filepath.Join("testdata", "input", inputFilename))
	if err != nil {
		log.Fatalf("error opening input file: %s", err)
	}
	defer input.Close()

	result, err := aoc24.Solve(day, input)
	if err != nil {
		log.Fatalf("error executing for day: %s", err)
	}

	output := `-- Day %02d --
Part 1: %s
Part 2: %s
`
	fmt.Printf(output, day, result.Part1, result.Part2)
}
