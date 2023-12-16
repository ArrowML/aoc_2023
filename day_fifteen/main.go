package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Lens struct {
	focal int
	slot  int
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
	var line string
	for fs.Scan() {
		line = fs.Text()
	}
	input.Close()

	//part1(line)
	part2(line)
}

func part1(sequence string) {

	var total int
	steps := strings.Split(sequence, ",")
	for _, s := range steps {
		total += getHashValue(s)
	}
	fmt.Println(total)
}

func part2(sequence string) {

	boxes := make(map[int]map[string]Lens)

	steps := strings.Split(sequence, ",")
	for _, s := range steps {
		if strings.Contains(s, "-") {
			ops := strings.Split(s, "-")
			label := ops[0]
			box_num := getHashValue(label)
			removed_slot := 0
			if b, ok := boxes[box_num][label]; ok {
				removed_slot = b.slot
				delete(boxes[box_num], label)
			} else {
				continue
			}

			if len(boxes[box_num]) > 0 {
				for k, b := range boxes[box_num] {
					if b.slot > removed_slot {
						boxes[box_num][k] = Lens{focal: b.focal, slot: b.slot - 1}
					}
				}
			}
		}

		if strings.Contains(s, "=") {
			ops := strings.Split(s, "=")
			label := ops[0]
			f, _ := strconv.Atoi(ops[1])
			box_num := getHashValue(label)
			if _, ok := boxes[box_num]; !ok {
				boxes[box_num] = map[string]Lens{label: {focal: f, slot: 1}}
			} else {
				s := len(boxes[box_num]) + 1
				if b, ok := boxes[box_num][label]; ok {
					s = b.slot
				}
				boxes[box_num][label] = Lens{focal: f, slot: s}
			}
		}
	}

	total := 0
	for bn, b := range boxes {
		fmt.Println(b)
		if len(b) > 0 {
			for _, l := range b {
				total += (bn + 1) * l.focal * l.slot
			}
		}
	}

	fmt.Println(total)

}

func getHashValue(hash string) int {
	multiplier := 17
	current := 0
	for _, c := range hash {
		current += int(c)
		current = (current * multiplier) % 256
	}
	return current
}
