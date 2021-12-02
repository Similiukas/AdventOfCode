package helpers

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

// Wrapper for converting string to int and ignoring error
func ToInt(input string) (result int) {
	result, _ = strconv.Atoi(input)
	return
}

// Wrapper for opening the day's input file, returning scanner and file to close
func FileScanner(day string) (r *bufio.Scanner, f *os.File) {
	file, err := os.Open(fmt.Sprintf("resources/%s/input.txt", day))

	if err != nil {
		panic(err)
	}
	// defer file.Close()

	return bufio.NewScanner(file), file
}
