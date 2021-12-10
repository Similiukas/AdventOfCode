package days

import (
	"sort"
	"strings"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

var incompleteLines [][]string

// Part one
func countCorruptionScore() (result int) {
	scanner, file := helpers.FileScanner("day10")
	defer file.Close()
	bracketMap := map[string]string{">": "<", "}": "{", "]": "[", ")": "("}
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "")
		stack := []string{}
		stack = append(stack, line[0])
		corrupted := false
		// Go through the brackets and add opening bracket to the stack
		// If the bracket is closing, then it must match the current top opening bracket
		// If they do, then we pop the opening bracket and move on
		for _, bracket := range line[1:] {
			if bracket == "<" || bracket == "(" || bracket == "{" || bracket == "[" {
				stack = append(stack, bracket)
			} else if bracketMap[bracket] == stack[len(stack)-1] {
				stack = stack[0 : len(stack)-1]
			} else {
				corrupted = true
				if bracket == ">" {
					result += 25137
				} else if bracket == "}" {
					result += 1197
				} else if bracket == "]" {
					result += 57
				} else {
					result += 3
				}
				break
			}
		}
		// Adding non-corrupted lines for part two
		if !corrupted {
			incompleteLines = append(incompleteLines, line)
		}
	}
	return
}

// Part two
func fixIncompleteLines() int {
	var results []int
	bracketScores := map[string]int{"(": 1, "[": 2, "{": 3, "<": 4}
	for _, line := range incompleteLines {
		stack := []string{}
		stack = append(stack, line[0])
		autocompleteScore := 0
		// Fill the stack
		for _, bracket := range line[1:] {
			if bracket == "<" || bracket == "(" || bracket == "{" || bracket == "[" {
				stack = append(stack, bracket) // stack add
			} else {
				stack = stack[0 : len(stack)-1] // stack pop
			}
		}
		// Rebuilding by going the stack backwards
		for i := len(stack) - 1; i >= 0; i-- {
			autocompleteScore = autocompleteScore*5 + bracketScores[stack[i]]
		}
		results = append(results, autocompleteScore)
	}
	sort.Slice(results, func(i, j int) bool { return results[i] < results[j] })
	return results[len(results)/2]
}

func Day10() {
	println("Hey, last day, lots of things to do, feeling excited and just ah")
	println("Corruption:", countCorruptionScore())
	println("Not too difficult day finish:", fixIncompleteLines())
}
