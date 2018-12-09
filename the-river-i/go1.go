package main

import (
	"fmt"
	"strconv"
)

func main() {
	var r1 int
	fmt.Scan(&r1)
	var r2 int
	fmt.Scan(&r2)

	for r1 != r2 {
		if r1 > r2 {
			r1, r2 = r2, r1
		}
		step := 0
		for _, r := range strconv.Itoa(r1) {
			n, _ := strconv.Atoi(string(r))
			step += n
		}
		r1 += step
	}

	fmt.Println(r1)
}
