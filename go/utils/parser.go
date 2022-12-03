package utils

import (
	"bufio"
	"os"
)

type Line_parser interface {
	Next() (line string, done bool, err error)
}

type file_line_parser struct {
	file    *os.File
	scanner *bufio.Scanner
}

func Parse_lines(path string) (file_line_parser, error) {
	var parser file_line_parser

	input_file, err := os.Open(path)
	if err != nil {
		return parser, err
	}

	scanner := bufio.NewScanner(input_file)
	scanner.Split(bufio.ScanLines)
	parser = file_line_parser{
		file:    input_file,
		scanner: scanner,
	}

	return parser, nil
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
