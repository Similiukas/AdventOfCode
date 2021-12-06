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

// Waaay faster and better part one and two
func properFishCount(fishies []int, days int) (result int) {
	// We get (0, 1, 2, 3, 4, 5, 6, 7, 8) where ith position tells how many fish are there with timer i
	var fishTimers [9]int
	for _, fish := range fishies {
		fishTimers[fish]++
	}
	for i := 0; i < days; i++ {
		newFishCount := fishTimers[0]
		// Every day, we move fish count from right to left to indicate that timer decreased by 1
		for j := 0; j < 8; j++ {
			fishTimers[j] = fishTimers[j+1]
		}
		fishTimers[8] = newFishCount  // There are n new number of fishes
		fishTimers[6] += newFishCount // Fish with timer 0 refresh to timer 6 and also fish with timer 7 are added
	}
	// Now we just calculate the result
	for _, fishCount := range fishTimers {
		result += fishCount
	}
	return
}

func Day06() {
	println("Systems done, now just hyped work work work")
	fishies := getInitialFishTimers()
	// println(strings.Split(helpers.GetFirstFileLine("day06"), ",")[4])
	// println("Number of fishies:", len(simulateFishDay(80, fishies)))
	// println("Fancy way:", getAllFishNumbers(fishies))
	println("Proper way:", properFishCount(fishies, 256))
}
