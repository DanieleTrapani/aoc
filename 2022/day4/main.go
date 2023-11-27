package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	result := 0

	for scanner.Scan() {
		// do stuff
		first, second := strings.Split(scanner.Text(), ",")[0], strings.Split(scanner.Text(), ",")[1]
		firstStart, firstEnd := strings.Split(first, "-")[0], strings.Split(first, "-")[1]
		secondStart, secondEnd := strings.Split(second, "-")[0], strings.Split(second, "-")[1]

		firstStartAsInt, _ := strconv.Atoi(firstStart)
		firstEndAsInt, _ := strconv.Atoi(firstEnd)
		secondStartAsInt, _ := strconv.Atoi(secondStart)
		secondEndAsInt, _ := strconv.Atoi(secondEnd)

		if (firstStartAsInt <= secondStartAsInt && firstEndAsInt >= secondStartAsInt) || (firstStartAsInt >= secondStartAsInt && firstStartAsInt <= secondEndAsInt) {
			result += 1
		}
	}
	fmt.Println(result)
}
