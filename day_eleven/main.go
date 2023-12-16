package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

type grid struct {
	grid [][]rune
}

type galaxy struct {
	x int
	y int
}

const (
	SPACE  = '.'
	GALAXY = '#'
)

func (g *grid) getPointValue(x, y int) rune {
	return g.grid[y][x]
}

func (g *grid) check_row_space(idx int) bool {
	empty := true
	for _, c := range g.grid[idx] {
		if c != SPACE {
			empty = false
			break
		}
	}
	return empty
}

func (g *grid) check_column_space(idx int) bool {
	empty := true
	for _, row := range g.grid {
		if row[idx] != SPACE {
			empty = false
			break
		}
	}
	return empty
}

func main() {
	pwd, _ := os.Getwd()
	input, err := os.Open(pwd + "/input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	fs := bufio.NewScanner(input)
	fs.Split(bufio.ScanLines)

	var intital grid
	for fs.Scan() {
		line := fs.Text()
		var row []rune
		for _, c := range line {
			row = append(row, c)
		}
		intital.grid = append(intital.grid, row)
	}
	input.Close()

	//part1(&intital)
	part2(&intital)
}

func insert_row(g *grid, idx int) grid {
	var row []rune
	for i := 0; i < len(g.grid[0]); i++ {
		row = append(row, SPACE)
	}
	g.grid = slices.Insert(g.grid, idx, row)
	return *g
}

func insert_column(g *grid, idx int) grid {
	for i := range g.grid {
		g.grid[i] = slices.Insert(g.grid[i], idx, SPACE)
	}
	return *g
}

func part1(inital *grid) {

	var insert_rows []int
	var insert_cols []int
	for i := range inital.grid {
		if inital.check_row_space(i) {
			insert_rows = append(insert_rows, i)
		}
	}
	for j := range inital.grid[0] {
		if inital.check_column_space(j) {
			insert_cols = append(insert_cols, j)
		}
	}

	var expanded grid
	for c, idx := range insert_rows {
		expanded = insert_row(inital, idx+c)
	}
	for c, idx := range insert_cols {
		expanded = insert_column(&expanded, idx+c)
	}

	var galaxies []galaxy
	for i, row := range inital.grid {
		for j, p := range row {
			if p == GALAXY {
				galaxies = append(galaxies, galaxy{x: j, y: i})
			}
		}
	}

	var distances []int
	for i := range galaxies {
		subset := galaxies[i+1:]
		for _, galaxy := range subset {
			distances = append(distances, get_distance(galaxies[i], galaxy))
		}
	}

	total := 0
	for _, dist := range distances {
		total += dist
	}
	fmt.Println(total)
}

func part2(inital *grid) {

	var insert_rows []int
	var insert_cols []int
	for i := range inital.grid {
		if inital.check_row_space(i) {
			insert_rows = append(insert_rows, i)
		}
	}
	for j := range inital.grid[0] {
		if inital.check_column_space(j) {
			insert_cols = append(insert_cols, j)
		}
	}

	var galaxies []galaxy
	for i, row := range inital.grid {
		for j, p := range row {
			if p == GALAXY {
				galaxies = append(galaxies, galaxy{x: j, y: i})
			}
		}
	}

	var distances []int
	for i := range galaxies {
		subset := galaxies[i+1:]
		for _, galaxy := range subset {
			distances = append(distances, get_distance_expansion(galaxies[i], galaxy, insert_rows, insert_cols))
		}
	}

	total := 0
	for _, dist := range distances {
		total += dist
	}
	fmt.Println(total)
}

func get_distance(one, two galaxy) int {
	var x_diff int
	if one.x >= two.x {
		x_diff = one.x - two.x
	} else {
		x_diff = two.x - one.x
	}
	y_diff := two.y - one.y
	return x_diff + y_diff
}

func get_distance_expansion(one, two galaxy, ir, ic []int) int {
	multiplier := 1000000 - 1

	x_expand := 0
	for _, r := range ic {
		if r > one.x && r < two.x {
			x_expand++
			continue
		}
		if r > two.x && r < one.x {
			x_expand++
		}
	}

	y_expand := 0
	for _, r := range ir {
		if r > one.y && r < two.y {
			y_expand++
		}
	}

	var x_diff int
	if one.x >= two.x {
		x_diff = one.x - two.x + (multiplier * x_expand)
	} else {
		x_diff = two.x - one.x + (multiplier * x_expand)
	}

	y_diff := two.y - one.y + (multiplier * y_expand)

	return x_diff + y_diff
}
