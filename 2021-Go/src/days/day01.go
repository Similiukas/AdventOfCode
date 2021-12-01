package days

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// Part one
func getHeightDifferenceCount() int {
	count := 0
	file, err := os.Open("resources/day01/input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	// optionally, resize scanner's capacity for lines over 64K, see next example
	previousHeight := 0
	height := 0
	for scanner.Scan() {
		height, err = strconv.Atoi(scanner.Text())
		fmt.Println(height, height > previousHeight && previousHeight != 0)
		if height > previousHeight && previousHeight != 0 {
			count++
		}
		previousHeight = height
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return count
}

func getTrippleHeightDiff() (count int) {
	// Reading the whole file at once, maybe not the best practice
	file, err := os.ReadFile("resources/day01/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	lines := strings.Split(string(file), "\n")
	for i := 0; i < len(lines)-3; i++ {
		num1, _ := strconv.Atoi(lines[i])
		num4, _ := strconv.Atoi(lines[i+3])
		if num4 > num1 {
			count++
		}
	}
	return
}

func Day01() {
	fmt.Println("You know, I did kinda miss it. It's just feels so good!")
	// count := getHeightDifferenceCount()
	// fmt.Println("Number of differences", count)
	fmt.Println("Part two:", getTrippleHeightDiff())
}
