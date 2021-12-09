package days

import (
	"sort"
	"strings"

	helpers "github.com/Similiukas/AdventOfCode/2021-Go/src"
)

func checkAdjacent(row int, column int, grid [][]string, victim string) bool {
	bigger, checked := 0, 0
	for _, r := range []int{-1, 0, 1} {
		for _, c := range []int{-1, 0, 1} {
			// Cannot be (0,0) or diagonal (r or c != o), or out of bounds
			if (r == 0 && c == 0) || (r != 0 && c != 0) || (row+r >= 100 || column+c >= 100 || row+r < 0 || column+c < 0) {
				continue
			}
			if grid[row+r][column+c] > victim {
				bigger++
			}
			checked++
		}
	}
	if checked == bigger {
		return true
	} else {
		return false
	}
}

// Storing coordinates of the low points
var lowPoints [][2]int

// Part one
func getLowPoints() (result int) {
	var grid [][]string

	scanner, file := helpers.FileScanner("day09")
	defer file.Close()
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "")
		grid = append(grid, line)
	}
	for row := range grid {
		for column := range grid[row] {
			if checkAdjacent(row, column, grid, grid[row][column]) {
				result += helpers.ToInt(grid[row][column]) + 1
				lowPoints = append(lowPoints, [2]int{row, column}) // Saving low point coordinates
			}
		}
	}
	return
}

// Part two (looked up the algorithm, just tried implementing queue in go)
func someFancyBFSFlodfillAlgo() int {
	var grid [][]string
	scanner, file := helpers.FileScanner("day09")
	defer file.Close()
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "")
		grid = append(grid, line)
	}

	// This is a Breath-First Search algorithm. Just looked up the answer, didn't come up with it myself
	// The idea is to go through the low points, create a queue and then check it's neighbors
	// If the neighbor is not 9 and hasn't been visited yet (by that low point)
	// We add that neighbor to the queue and repeat
	// https://www.geeksforgeeks.org/breadth-first-search-or-bfs-for-a-graph/
	// https://www.reddit.com/r/adventofcode/comments/rca6vp/comment/hntgdf4/?utm_source=share&utm_medium=web2x&context=3
	basins := []int{}
	for _, lowPoint := range lowPoints {
		queue := [][2]int{}
		queue = append(queue, lowPoint) // We create a queue with the lowPoint
		visitedPoints := []int{}        // A set of visited points
		for len(queue) != 0 {           // While q:
			point := queue[0]
			queue = queue[1:] // Essentially just point = queue.pop() in two lines
			// Go through all the neighbors of `point` and if they are not a 9
			// Or haven't been visited yet, add them to the queue
			for _, r := range []int{-1, 0, 1} {
				for _, c := range []int{-1, 0, 1} {
					neighbor := []int{point[0] + r, point[1] + c}
					// If we are out of bounds, diagonal or (0,0)
					if (r == 0 && c == 0) || (r != 0 && c != 0) || (neighbor[0] >= len(grid) || neighbor[1] >= len(grid[0]) || neighbor[0] < 0 || neighbor[1] < 0) {
						continue
					}
					// If the neighbor is 9 or we already visited this point, we skip it
					if grid[neighbor[0]][neighbor[1]] == "9" || helpers.ContainsInt(visitedPoints, neighbor[0]*100+neighbor[1]) {
						continue
					}
					// New point which is not a 9 and hasn't been visited yet
					visitedPoints = append(visitedPoints, neighbor[0]*100+neighbor[1])
					queue = append(queue, [2]int{neighbor[0], neighbor[1]})
				}
			}
		}
		basins = append(basins, len(visitedPoints))
	}
	sort.Slice(basins, func(i, j int) bool { return basins[i] > basins[j] })
	return basins[0] * basins[1] * basins[2]
}

func Day09() {
	println("Not gonna lie, don't really wanna go to litsoc and just wanna finish work")
	println("A bit of borrowing from dart", getLowPoints())
	println("This time, just learning go", someFancyBFSFlodfillAlgo())
}
