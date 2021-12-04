package days

import (
	"log"
	"os"
	"strings"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

var markingTotem = "#"

func getCallingNumbers() []string {
	// https://stackoverflow.com/questions/24972950/go-convert-strings-in-array-to-integer
	return strings.Split(helpers.GetFirstFileLine("day04"), ",")
}

// Construct all the boards
func getBoards() (boards [][][]string) {
	file, err := os.ReadFile("resources/day04/input.txt")
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(file), "\n")

	boards = make([][][]string, 0) // 100 5x5 boards

	for i := 2; i < len(lines); i += 6 {
		board := [][]string{} // Start constructing a board (same as make([][]string, 0))
		for j := 0; j < 5; j++ {
			nums := strings.Fields(lines[i+j]) // Slice a row of numbers into a slice
			board = append(board, nums)        // Add that slice to the board
		}
		boards = append(boards, board) // Add constructed board to the list
	}

	return
}

// If element is in the board, we mark it
func replaceIfExists(board [][]string, elem string) {
	for i := range board {
		for j := range board[i] {
			if board[i][j] == elem {
				board[i][j] = markingTotem
			}
		}
	}
}

// Check if the board won
func checkForWinner(board [][]string) bool {
	for i := range board { // Checking rows
		if board[i][0] == markingTotem && board[i][1] == markingTotem &&
			board[i][2] == markingTotem && board[i][3] == markingTotem && board[i][4] == markingTotem {
			return true
		}
	}
	for i := range board { // Checking columns
		if board[0][i] == markingTotem && board[1][i] == markingTotem &&
			board[2][i] == markingTotem && board[3][i] == markingTotem && board[4][i] == markingTotem {
			return true
		}
	}
	return false
}

// Calculate the rest of the board numbers sum
func restOfBoardNums(board [][]string) (result int) {
	result = 0
	for i := range board {
		for j := range board {
			if board[i][j] != markingTotem {
				result += helpers.ToInt(board[i][j])
			}
		}
	}
	return
}

// Part one
func playTheGame() int {
	// gaunam skaicius
	// gaunam board list
	// einam per skaicius ir tikrinam ar tas skaicius yra boarde
	// jei yra boarde, pakeiciam ji i -1 (pazymim)
	// tikrinam ar board turi uzbaigta row ar column
	// jei turi, tai tada suskaiciuojam likusiu board skaicius ir dauginam is skaiciaus
	luckyNumbers := getCallingNumbers()
	boards := getBoards()
	for _, number := range luckyNumbers {
		for _, board := range boards {
			replaceIfExists(board, number)
			if checkForWinner(board) {
				return helpers.ToInt(number) * restOfBoardNums(board)
			}
		}
	}
	return 0
}

// Part two
func nowLetsLose() (result int) {
	// If board wins, then we calculate the result and mark the board as winner
	// Continue the game until the we end the lucky numbers
	// Return the result
	result = 0
	luckyNumbers := getCallingNumbers()
	boards := getBoards()
	for _, number := range luckyNumbers {
		for _, board := range boards {
			if board[0][0] == "W" { // If the board already won, we don't care about it
				continue
			}
			replaceIfExists(board, number)
			if checkForWinner(board) {
				result = helpers.ToInt(number) * restOfBoardNums(board)
				board[0][0] = "W"
			}
		}
	}
	return
}

func Day04() {
	println("Starting today late but hey, will have a call later with hihi")
	// println(getCallingNumbers())
	// getBoards()
	println("Result:", playTheGame())
	println("Lucky loser:", nowLetsLose())
}
