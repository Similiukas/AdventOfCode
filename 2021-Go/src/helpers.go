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

// Returns only the first line of the file
func GetFirstFileLine(day string) string {
	file, err := os.Open(fmt.Sprintf("resources/%s/input.txt", day))

	if err != nil {
		panic(err)
	}
	defer file.Close()
	reader := bufio.NewReader(file)
	firstLine, err := reader.ReadString('\n')
	if err != nil {
		panic(err)
	}

	return firstLine[:len(firstLine)-1]
}

// Removes element at i-th position replacing it with the last element of the slice
// (not order preserving)
func RemoveFromArray(s []string, i int) []string {
	s[i] = s[len(s)-1]
	return s[:len(s)-1]
}

func SliceStringToInt(arr []string) (result []int) {
	for _, i := range arr {
		num, err := strconv.Atoi(i)
		if err != nil {
			panic(err)
		}
		result = append(result, int(num))
	}
	return
}

func ContainsString(arr []string, victim string) bool {
	for _, a := range arr {
		if a == victim {
			return true
		}
	}
	return false
}

// Yep, same thing, but now with ints. A bit dry but eh, just wanna finish this for today
func ContainsInt(arr []int, victim int) bool {
	for _, a := range arr {
		if a == victim {
			return true
		}
	}
	return false
}

// No idea how to implement a new data structure with functions
// func CreateQueue() *[]int {
// 	var queue []int
// 	return &queue
// }

// type queue interface {
// 	q() []int
// }

// func queueTop(q queue) int {
// 	return q.q[0]
// }

// type queueTop func(queue) int

type Queue struct {
	q   []int
	top func(Queue) int
}
