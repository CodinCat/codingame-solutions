package main

import "fmt"
import "os"
import "bufio"

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Buffer(make([]byte, 1000000), 1000000)

	scanner.Scan()
	order := scanner.Text()
	scanner.Scan()
	side := scanner.Text()

	result := make(map[string]int)
	result["R"] = 1
	result["L"] = 1
	result["U"] = 1
	result["D"] = 1

	for _, char := range order {
		switch char {
		case 'R':
			result["L"] += result["R"]
			result["R"] = 1
			result["U"] *= 2
			result["D"] *= 2
		case 'L':
			result["R"] += result["L"]
			result["L"] = 1
			result["U"] *= 2
			result["D"] *= 2
		case 'U':
			result["D"] += result["U"]
			result["U"] = 1
			result["R"] *= 2
			result["L"] *= 2
		case 'D':
			result["U"] += result["D"]
			result["D"] = 1
			result["R"] *= 2
			result["L"] *= 2
		}
	}

	fmt.Println(result[side])
}
