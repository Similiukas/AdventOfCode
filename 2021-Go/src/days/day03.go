package days

import (
	"log"
	"math"
	"os"
	"strconv"
	"strings"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

// Part one
func splitBitsAndCount() (result int) {
	scanner, file := helpers.FileScanner("day03")
	defer file.Close()
	var bitsArray [12]int
	for scanner.Scan() {
		line := scanner.Text()
		for i, c := range line {
			if c == 48 { // If bit is "0"
				bitsArray[i] -= 1
			} else {
				bitsArray[i] += 1
			}
		}
	}
	gammaRate, epsilonRate := 0, 0
	for i, bit := range bitsArray {
		if bit > 0 { // If bit is "1"
			gammaRate += int(math.Pow(2, float64(len(bitsArray)-i-1)))
		} else {
			epsilonRate += int(math.Pow(2, float64(len(bitsArray)-i-1)))
		}
	}
	return gammaRate * epsilonRate
}

func calculateHarderLogic(arr []string, leastCommon bool) (answer int64) {
	// Properly copying a slice (https://go.dev/blog/slices-intro)
	leftBitsArray := make([]string, len(arr))
	copy(leftBitsArray, arr)
	iteration := 0 // Keeping track of which bit we are looking at
	// This is a while loop
	for len(leftBitsArray) != 1 && iteration != 12 {
		numberOfOnes := 0
		// Counting the number of ones appear in iteration-th bit
		for i := 0; i < len(leftBitsArray); i++ {
			if leftBitsArray[i][iteration] == 49 {
				numberOfOnes++
			}
		}

		var toRemove byte = 48
		// Determining which bit needs to be removed (48 -> 0;  49 -> 1)
		// There are numberOfOnes of 1s' and number of zeros are the rest in the array
		if (leastCommon && numberOfOnes >= len(leftBitsArray)-numberOfOnes) ||
			(!leastCommon && numberOfOnes < len(leftBitsArray)-numberOfOnes) {
			toRemove = 49
		}
		// Removing from the array bits which have 1 or 0 in iteration-th position
		for i := 0; i < len(leftBitsArray); i++ {
			if leftBitsArray[i][iteration] == toRemove {
				leftBitsArray = helpers.RemoveFromArray(leftBitsArray, i)
				i-- // Since removing puts last element at the start, we need to check element at i-th position again
			}
		}
		iteration++
	}
	answer, _ = strconv.ParseInt(leftBitsArray[0], 2, 64)
	return
}

// Part two
func harderBitLogic() int64 {
	file, err := os.ReadFile("resources/day03/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	allBits := strings.Split(string(file), "\n")
	oxygen, co2 := calculateHarderLogic(allBits, false), calculateHarderLogic(allBits, true)

	return oxygen * co2
}

func Day03() {
	// Note for test input:
	// Part one only works for inputs which all have 12 bit integers
	println("Aight, now even before breakfast", 125*3860)
	println("Result:", splitBitsAndCount())
	println("Pagrazintas siek tiek part two:", harderBitLogic())
}

// 126 3202
// 125 3856
// 125 3860
