package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("data.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	var lines []string
	var columns [][]string

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
	}

	lineLength := len(lines[0])

	// Initialize the columns slice
	columns = make([][]string, lineLength)

	// Split each line into columns
	for _, line := range lines[0:8] {
		for i, char := range line {
			columns[i] = append(columns[i], string(char))
		}
	}

	// Print the columns
	for _, col := range columns {
		fmt.Println(strings.Join(col, " "))
	}

	crates := lines[0:8]

	for _, line := range crates {
		fmt.Println(line)
	}
}
