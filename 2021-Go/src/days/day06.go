package days

import (
	"math"
	"strings"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

func getInitialFishTimers() []int {
	return helpers.SliceStringToInt(strings.Split(helpers.GetFirstFileLine("day06"), ","))
}

// Part one
func simulateFishDay(day int, fishes []int) []int {
	if day == 0 {
		return fishes
	}

	for i, fish := range fishes {
		if fish == 0 {
			fishes = append(fishes, 8)
			fishes[i] = 7
		}
		fishes[i]--
	}
	return simulateFishDay(day-1, fishes)
}

func calculateFishKids(daysLeft int, fishTimer int) int {
	if daysLeft < 0 {
		return 0
	}
	// Calculating how many kids this fish will have
	numberOfKids := int(math.Floor(float64(daysLeft-fishTimer-1)/7 + 1.0))
	if numberOfKids < 0 {
		return 0
	}
	grandkids := 0 // Going through all the kids and calculating how many kids they will have
	for i := 0; i < numberOfKids; i++ {
		grandkids += calculateFishKids(daysLeft-fishTimer-7*i-1, 8)
	}
	return numberOfKids + grandkids
}

// Part two
func getAllFishNumbers(fishies []int) (result int) {
	possibilities := make(map[int]int)
	for _, fish := range fishies {
		if possibilities[fish] != 0 { // If we already calculated for this fish timer, we can skip it
			result += possibilities[fish]
		} else {
			kids := calculateFishKids(256, fish)
			possibilities[fish] = kids
			result += kids
		}
	}
	return result + len(fishies)
}

func Day06() {
	println("Systems done, now just hyped work work work")
	fishies := getInitialFishTimers()
	// println(strings.Split(helpers.GetFirstFileLine("day06"), ",")[4])
	println("Number of fishies:", len(simulateFishDay(80, fishies)))
	println("Fancy way:", getAllFishNumbers(fishies))
}
