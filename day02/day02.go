package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"

	"github.com/charmbracelet/log"
)

func checkSafety(nums []int) bool {
	if len(nums) < 2 {
		return true
	}

	increasing := nums[1] > nums[0]

	for i := 1; i < len(nums); i++ {
		diff := nums[i] - nums[i-1]

		if diff == 0 || abs(diff) > 3 {
			return false
		}

		if increasing && diff < 0 {
			return false
		}

		if !increasing && diff > 0 {
			return false
		}
	}
	return true
}

func checkSafetyOneBad(nums []int) bool {
	if checkSafety(nums) {
		return true
	}

	for i := 0; i < len(nums); i++ {
		tmp := make([]int, 0, len(nums)-1)
		tmp = append(tmp, nums[:i]...)
		tmp = append(tmp, nums[i+1:]...)

		if checkSafety(tmp) {
			return true
		}
	}

	return false
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	f, err := os.Open("./day_02.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	safeReports := 0
	for scanner.Scan() {
		line := scanner.Text()

		numStrings := strings.Fields(line)
		numbers := make([]int, len(numStrings))

		for i, numStr := range numStrings {
			num, err := strconv.Atoi(numStr)
			if err != nil {
				log.Error(err)
				continue
			}

			numbers[i] = num
		}

		if checkSafetyOneBad(numbers) {
			safeReports++
		}

		if err := scanner.Err(); err != nil {
			log.Fatal(err)
		}

	}

	log.Infof("Number of safe reports %v", safeReports)
}
