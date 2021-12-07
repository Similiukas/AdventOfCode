package days

import (
	"math"
	"strings"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

// Part one and two
func someWeirdMath(partOne bool) float64 {
	// Need to solve for |coord[0]-x| + |coord[1]-x| + ... + |coord[n]-x| = fuel
	// Then the lowest possible fuel amount is the answer
	// For part two, the x is an arithmetic progression: x = 1 + 2 + ... + x_m = m*(m+1)/2
	// Hence need to solve for |coord[0]-x|*(|coord[0]-x|+1)/2 + ... + |coord[n]-x|*(|coord[n]-x|+1)/2 = fuel

	// To minimze the sum of absolute values median can be used: https://math.stackexchange.com/q/113270
	// Lots of people solved this way by just guessing, interesting https://www.reddit.com/r/adventofcode/comments/rars4g/comment/hnk7n2z/?utm_source=share&utm_medium=web2x&context=3
	// For part two, mean is really close for the optimal solution, however it works only for n^2 and not n(n+1)
	// But the error is usually really small (<0.3) https://www.reddit.com/r/adventofcode/comments/rar7ty/comment/hnk6gz0/?utm_source=share&utm_medium=web2x&context=3
	coords := helpers.SliceStringToInt(strings.Split(helpers.GetFirstFileLine("day07"), ","))
	fuel := math.MaxFloat64
	for x := 0; x < 1200; x++ {
		result := 0.0
		for _, coord := range coords {
			if partOne {
				result += math.Abs(float64(coord - x))
			} else {
				result += (math.Abs(float64(coord-x)) * (math.Abs(float64(coord-x)) + 1)) / 2
			}
		}

		if result < fuel {
			fuel = result
			// Essentially these are just linear equations and we're looking for the smallest value
			// Hence if the result is increasing, we found the minimum value
		} else if fuel != math.MaxFloat64 && result > fuel {
			return fuel
		}
	}
	return fuel
}

func Day07() {
	println("Cold day but the final prep is starting")
	println("Does math work?", int(someWeirdMath(true)))
	println("Some fancy lucky math finding:", int(someWeirdMath(false)))
}
