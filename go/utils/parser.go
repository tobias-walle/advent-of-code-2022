package utils

import (
	"bufio"
	"fmt"
	"os"
)

type LineParser interface {
	Close()
	Next() (line string, done bool, err error)
}

type FileLineParser struct {
	file    *os.File
	scanner *bufio.Scanner
}

func (p FileLineParser) Close() {
	p.file.Close()
}

func (p FileLineParser) Next() (line string, done bool, err error) {
	if p.scanner.Scan() {
		return p.scanner.Text(), false, p.scanner.Err()
	}
	defer p.Close()
	return "", true, p.scanner.Err()
}

func ParseLines(path string) (FileLineParser, error) {
	input_file, err := os.Open(path)
	if err != nil {
		return FileLineParser{}, err
	}

	scanner := bufio.NewScanner(input_file)
	scanner.Split(bufio.ScanLines)
	parser := FileLineParser{
		file:    input_file,
		scanner: scanner,
	}

	return parser, nil
}

func ParseInputFileLinesFromArgs() (FileLineParser, error) {
	input_file_path := os.Args[1]
	if input_file_path == "" {
		panic("Please provide the path of the input file as the first argument")
	}

	fmt.Println("Read", input_file_path)
	return ParseLines(input_file_path)
}
