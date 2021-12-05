package days

import (
	"math"
	"regexp"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

// Part one and two
func countOverlap(partTwo bool) (result int) {
	result = 0
	board := [1000][1000]int{}
	scanner, file := helpers.FileScanner("day05")
	defer file.Close()

	r := regexp.MustCompile("(\\d+),(\\d+) -> (\\d+),(\\d+)")
	for scanner.Scan() {
		line := scanner.Text()
		values := r.FindStringSubmatch(line)
		x1, x2 := helpers.ToInt(values[1]), helpers.ToInt(values[3])
		y1, y2 := helpers.ToInt(values[2]), helpers.ToInt(values[4])
		if x1 == x2 {
			smaller, larger := y1, y2
			if smaller > larger { // Swapping values if necessary
				smaller, larger = larger, smaller
			}

			for i := smaller; i <= larger; i++ {
				board[i][x1]++
			}
		} else if y1 == y2 {
			smaller, larger := x1, x2
			if smaller > larger {
				smaller, larger = larger, smaller
			}

			for i := smaller; i <= larger; i++ {
				board[y1][i]++
			}
		} else if partTwo { // Diagonal
			x, y := 0, 0
			for i := 0; i <= int(math.Abs(float64(x2-x1))); i++ {
				// Would've been so easy with ternary operator but nah, go devs said ugly
				// That's honestly, just stupid excuse https://stackoverflow.com/a/19979289
				// board[increaseY ?? y1+i : y1-i][inincreaseX ?? x1+i : x1-i]++
				if y1 < y2 {
					y = y1 + i
				} else {
					y = y1 - i
				}
				if x1 < x2 {
					x = x1 + i
				} else {
					x = x1 - i
				}
				board[y][x]++
			}
		}
	}
	// Counting overlapping
	for i := range board {
		for j := range board[i] {
			if board[i][j] > 1 {
				result++
			}
		}
	}

	return
}

func Day05() {
	println("Hey, less than a week left, woop woop")
	println("Simple overlap:", countOverlap(false))
	println("Diagonal overlap:", countOverlap(true))
}
