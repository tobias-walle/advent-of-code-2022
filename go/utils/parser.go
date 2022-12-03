package utils

import (
	"bufio"
	"fmt"
	"os"
)

type Line_parser interface {
	Next() (line string, done bool, err error)
}

type file_line_parser struct {
	file    *os.File
	scanner *bufio.Scanner
}

func (p file_line_parser) Close() {
	p.file.Close()
}

func (p file_line_parser) Next() (line string, done bool, err error) {
	if p.scanner.Scan() {
		return p.scanner.Text(), false, p.scanner.Err()
	}
	defer p.Close()
	return "", true, p.scanner.Err()
}

func Parse_lines(path string) (file_line_parser, error) {
	input_file, err := os.Open(path)
	if err != nil {
		return file_line_parser{}, err
	}

	scanner := bufio.NewScanner(input_file)
	scanner.Split(bufio.ScanLines)
	parser := file_line_parser{
		file:    input_file,
		scanner: scanner,
	}

	return parser, nil
}

func Parse_input_file_lines_from_args() (file_line_parser, error) {
	input_file_path := os.Args[1]
	if input_file_path == "" {
		panic("Please provide the path of the input file as the first argument")
	}

	fmt.Println("Read", input_file_path)
	return Parse_lines(input_file_path)
}
