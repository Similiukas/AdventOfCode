package days

import (
	"math"
	"regexp"
	"strings"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

// Part one
func simpleSegmentStuff() (result int) {
	scanner, file := helpers.FileScanner("day08")
	defer file.Close()
	r := regexp.MustCompile("([^|]) \\| (\\w+) (\\w+) (\\w+) (\\w+)")
	for scanner.Scan() {
		line := scanner.Text()
		reg := r.FindStringSubmatch(line)
		for i := 2; i <= 5; i++ {
			if len(reg[i]) == 2 || len(reg[i]) == 4 || len(reg[i]) == 3 || len(reg[i]) == 7 {
				result++
			}
		}
	}
	return
}

func decodeSegment(one string, four string, victim string) int {
	// Would be way easier if you could check for example one is in victim, then you know victim is 3
	// But since the chars can be scrambled, doing it this way
	decodingLengthFive := len(victim) == 5
	if decodingLengthFive && strings.Contains(victim, one[0:1]) && strings.Contains(victim, one[1:2]) {
		return 3
	} else if decodingLengthFive {
		count := 0 // Counting how many char of four fit in the victim
		for i := 0; i < 4; i++ {
			if strings.Contains(victim, four[i:i+1]) {
				count++
			}
		}
		if count == 2 {
			return 2
		} else {
			return 5
		}
	} else if strings.Contains(victim, one[0:1]) && strings.Contains(victim, one[1:2]) {
		count := 0
		for i := 0; i < 4; i++ {
			if strings.Contains(victim, four[i:i+1]) {
				count++
			}
		}
		if count == 3 {
			return 0
		} else {
			return 9
		}
	} else {
		return 6
	}
}

// Part two
func nowTheHardSegments() (result int) {
	scanner, file := helpers.FileScanner("day08")
	defer file.Close()
	r := regexp.MustCompile("(\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) \\| (\\w+) (\\w+) (\\w+) (\\w+)")
	for scanner.Scan() {
		line := r.FindStringSubmatch(scanner.Text())
		semiResult := 0
		// Looking for 1 and 4
		one, four := "", ""
		for _, code := range line[1:11] {
			if len(code) == 2 {
				one = code
			} else if len(code) == 4 {
				four = code
			}
			if one != "" && four != "" {
				break
			}
		}
		// Building the number
		for i := 11; i <= 14; i++ {
			if len(line[i]) == 2 { // It's 1
				semiResult += int(math.Pow10(14 - i))
			} else if len(line[i]) == 4 { // It's 4
				semiResult += int(math.Pow10(14-i)) * 4
			} else if len(line[i]) == 3 { // It's 7
				semiResult += int(math.Pow10(14-i)) * 7
			} else if len(line[i]) == 7 { // It's 8
				semiResult += int(math.Pow10(14-i)) * 8
			} else { // Either 0, 6 or 9 or 2, 3 or 5
				semiResult += int(math.Pow10(14-i)) * decodeSegment(one, four, line[i])
			}
		}
		result += semiResult
	}
	return
}

func Day08() {
	println("Cleaning day, and hey, haskell might be going good")
	println("Not looking forward for part two", simpleSegmentStuff())
	println("So does this work?", nowTheHardSegments())
}
