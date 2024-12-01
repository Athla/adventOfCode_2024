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

func QuickSort(arr []int) []int {
	return quickSort(arr, 0, len(arr)-1)
}

func quickSort(arr []int, low, high int) []int {
	if low < high {
		arr, p := partition(arr, low, high)
		arr = quickSort(arr, 0, p-1)
		arr = quickSort(arr, p+1, high)
	}

	return arr
}

func partition(arr []int, low, high int) ([]int, int) {
	pivot := arr[high]
	i := low

	for j := low; j < high; j++ {
		if arr[j] < pivot {
			arr[i], arr[j] = arr[j], arr[i]
			i++
		}
	}

	arr[i], arr[high] = arr[high], arr[i]

	return arr, i
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
func main() {
	// Load file
	f, err := os.Open("day01.csv")
	if err != nil {
		log.Fatalf("Unable to read file due: %v", err)
	}
	defer f.Close()
	// Parse it into left and right sides
	csvReader := csv.NewReader(f)
	data, err := csvReader.ReadAll()
	if err != nil {
		log.Fatalf("Unable to read file due: %v", err)
	}
	// Sort it
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
	// Sort the two slices
	sort.Ints(left)
	sort.Ints(right)
	log.Infof("Left size: %v Right size: %v", len(left), len(right))
	// Do diff
	diff := 0
	for i := 0; i < 1000; i++ {
		diff += doDiff(left[i], right[i])
	}

	log.Info(diff)

	// Sum diff

}
