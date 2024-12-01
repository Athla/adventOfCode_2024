package main

import (
	"encoding/csv"
	"os"
	"sort"
	"strconv"

	"github.com/charmbracelet/log"
)

type Input struct {
	left  uint
	right uint
}

func doDiff(a, b int) int {
	if a > b {
		return (a - b)
	} else if b > a {
		return (b - a)
	} else {
		return 0
	}
}
func FirstHalf(data [][]string) int {
	var left []int
	var right []int
	for i := 0; i < len(data); i++ {
		vLeft, err := strconv.Atoi(data[i][0])
		if err != nil {
			log.Fatal(err)
		}
		left = append(left, vLeft)

		vRight, err := strconv.Atoi(data[i][1])
		if err != nil {
			log.Fatal(err)
		}
		right = append(right, vRight)
	}
	sort.Ints(left)
	sort.Ints(right)

	diff := 0
	for i := 0; i < 1000; i++ {
		diff += doDiff(left[i], right[i])
	}

	log.Info(diff)
	return diff
}
func main() {
	f, err := os.Open("day01.csv")
	if err != nil {
		log.Fatalf("Unable to read file due: %v", err)
	}
	defer f.Close()

	csvReader := csv.NewReader(f)
	data, err := csvReader.ReadAll()
	if err != nil {
		log.Fatalf("Unable to read file due: %v", err)
	}

	// Simlarity score
	// Figure how many times the left side appears in the right side
	// Multiply the number in the left side by the amount of times it appears in the right
	// map left:numApp in the right side, if the key already exists jsut skip for now
	var left []int
	var right []int
	for i := 0; i < len(data); i++ {
		vLeft, err := strconv.Atoi(data[i][0])
		if err != nil {
			log.Fatal(err)
		}
		left = append(left, vLeft)

		vRight, err := strconv.Atoi(data[i][1])
		if err != nil {
			log.Fatal(err)
		}
		right = append(right, vRight)
	}
	rightFreq := make(map[int]int)
	for _, num := range right {
		rightFreq[num]++
	}

	totalScore := 0
	for _, num := range left {
		totalScore += num * rightFreq[num]
	}

	log.Info(totalScore)
	// Check how many time s the left number appears in the right side
}
