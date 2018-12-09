package main

import (
	"fmt"
	"math"
)

func main() {
	var R int
	fmt.Scan(&R)
	var V int
	fmt.Scan(&V)
	robberTimes := make([]int, R)

	for i := 0; i < V; i++ {
		var C, N int
		fmt.Scan(&C, &N)
		time := int(math.Pow(10, float64(N)) * math.Pow(5, float64(C-N)))
		nextRobber := findIndexOfMin(robberTimes)
		robberTimes[nextRobber] += time
	}

	fmt.Println(findMaxTime(robberTimes))
}

func findIndexOfMin(arr []int) int {
	min := arr[0]
	index := 0
	for i, v := range arr {
		if v <= min {
			index = i
			min = v
		}
	}
	return index
}

func findMaxTime(times []int) int {
	max := 0
	for _, v := range times {
		if v > max {
			max = v
		}
	}
	return max
}
