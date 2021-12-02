package days

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

// Part one
func calculateFinalPosition() (result int) {
	file, err := os.Open("resources/day02/input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	r := regexp.MustCompile("(.*) (\\d+)")
	forward := 0
	depth := 0
	for scanner.Scan() {
		input := scanner.Text()
		reg := r.FindStringSubmatch(input)
		n, _ := strconv.Atoi(reg[2])
		switch reg[1] {
		case "forward":
			forward += n
			break
		case "down":
			depth += n
			break
		case "up":
			depth -= n
			break
		}

	}
	result = forward * depth
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return
}

func calculateHarderFinalPosition() (result int) {
	scanner, file := helpers.FileScanner("day02")
	defer file.Close()

	r := regexp.MustCompile("(.*) (\\d+)")
	horizontal, depth, aim := 0, 0, 0
	for scanner.Scan() {
		input := scanner.Text()
		reg := r.FindStringSubmatch(input)
		n, _ := strconv.Atoi(reg[2])
		switch reg[1] {
		case "forward":
			horizontal += n
			depth += aim * n
			break
		case "down":
			aim += n
			break
		case "up":
			aim -= n
			break
		}

	}
	result = horizontal * depth
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return
}

func Day02() {
	fmt.Println("A bit slow and sleepy")
	fmt.Println("Simple maths: ", calculateFinalPosition())
	fmt.Println("Isn't it just kinda the same?", calculateHarderFinalPosition())
}
