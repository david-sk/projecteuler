//
// Random connected area, v2
// https://projecteuler.net/problem=701
//
// go run src/problems/0701_random_connected_area/v2.go
//

package main

import (
	"math/rand"
	"time"
)

type Rect struct {
	Width   int
	Height  int
	Cells   [][]int
	Visited [][]bool
}

func (r *Rect) updateLocalArea(i, j, maxI, maxJ, value int) {
	if r.Visited[i][j] {
		return
	}
	r.Visited[i][j] = true
	r.Cells[i][j] = value
	if j > 0 && r.Cells[i][j-1] > 0 {
		r.updateLocalArea(i, j-1, maxI, maxJ, value)
	}
	if ((i < maxI && j+1 < r.Width) || (i == maxI && j+1 <= maxJ)) && r.Cells[i][j+1] > 0 {
		r.updateLocalArea(i, j+1, maxI, maxJ, value)
	}
	if i > 0 && r.Cells[i-1][j] > 0 {
		r.updateLocalArea(i-1, j, maxI, maxJ, value)
	}
	if i+1 <= maxI && (i+1 != maxI || j <= maxJ) && r.Cells[i+1][j] > 0 {
		r.updateLocalArea(i+1, j, maxI, maxJ, value)
	}
}

func (r *Rect) findArea() int {
	maxArea := 0
	for i := 0; i < r.Height; i++ {
		for j := 0; j < r.Width; j++ {
			if r.Cells[i][j] == 1 {
				above := 0
				if i > 0 && r.Cells[i-1][j] > 0 {
					above = r.Cells[i-1][j]
				}
				left := 0
				if j > 0 && r.Cells[i][j-1] > 0 {
					left = r.Cells[i][j-1]
				}
				if above > 0 {
					r.resetVisited()
					r.Cells[i][j] += above
					r.Visited[i][j] = true
					r.updateLocalArea(i-1, j, i, j, r.Cells[i][j])
				}
				if left > 0 && (above == 0 || !r.Visited[i][j-1]) {
					r.resetVisited()
					r.updateLocalArea(i, j, i, j, left+r.Cells[i][j])
				}
				if maxArea < r.Cells[i][j] {
					maxArea = r.Cells[i][j]
				}
			}
		}
	}
	return maxArea
}

func (r *Rect) resetVisited() {
	if len(r.Visited) == 0 {
		r.Visited = [][]bool{}
		for i := 0; i < r.Width; i++ {
			r.Visited = append(r.Visited, []bool{})
			for j := 0; j < r.Height; j++ {
				r.Visited[i] = append(r.Visited[i], false)
			}
		}
	} else {
		for i := 0; i < r.Width; i++ {
			for j := 0; j < r.Height; j++ {
				r.Visited[i][j] = false
			}
		}
	}
}

func (r *Rect) randomizeCells() {
	for i := 0; i < r.Width; i++ {
		for j := 0; j < r.Height; j++ {
			r.Cells[i][j] = rand.Intn(2)
		}
	}
}

func (r *Rect) display() {
	for i := 0; i < r.Width; i++ {
		for j := 0; j < r.Height; j++ {
			print(r.Cells[i][j], " ")
		}
		println()
	}
}

func try5by5() {
	cells := [][]int{
		{0, 1, 0, 0, 0},
		{0, 1, 0, 1, 0},
		{1, 1, 1, 1, 1},
		{1, 0, 1, 0, 1},
		{1, 1, 1, 0, 1},
	}
	rect := Rect{
		Cells:  cells,
		Width:  len(cells[0]),
		Height: len(cells),
	}
	rect.display()
	println("- - - -")
	println(rect.findArea())
	println("- - - -")
	rect.display()
	println("------------------")
}

func tryRandom2by2() {
	rand.Seed(time.Now().UnixNano())

	cells := [][]int{
		{0, 0},
		{0, 0},
	}
	rect := Rect{
		Cells:  cells,
		Width:  len(cells[0]),
		Height: len(cells),
	}

	numPossibilities := 16 * 10000
	maxAreaSum := 0
	for i := 0; i < numPossibilities; i++ {
		rect.randomizeCells()
		maxAreaSum += rect.findArea()
	}
	println("tryRandom2by2 result:")
	println(float64(maxAreaSum) / float64(numPossibilities))
}

func main() {
	try5by5()
	tryRandom2by2()
}
