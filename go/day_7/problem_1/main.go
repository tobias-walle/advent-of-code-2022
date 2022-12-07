package main

import (
	"fmt"
	"strconv"
	"strings"
	"tobias-walle/aoc-22/utils"
)

func main() {
	lines, err := utils.ParseInputFileLinesFromArgs()
	utils.PanicOnErr(err)
	defer lines.Close()

	result, err := getResult(lines)
	utils.PanicOnErr(err)
	println(result)
}

func getResult(lines utils.LineParser) (int, error) {
	root_node := NewDir("/", nil)
	currentNode := &root_node
	for {
		line, done, err := lines.Next()
		if err != nil {
			return 0, err
		}
		if done {
			break
		}

		parts := strings.Split(line, " ")
		if strings.HasPrefix(line, "$ cd") {
			targetDir := parts[2]
			if targetDir == "/" {
				continue
			} else if targetDir == ".." {
				currentNode = (*currentNode).Parent()
			} else {
				child := findChild(*currentNode, targetDir)
				if child == nil {
					newDir := NewDir(targetDir, currentNode)
					(*currentNode).AddChild(&newDir)
					child = &newDir
				}
				currentNode = child
			}
		} else if strings.HasPrefix(line, "$ ls") {
		} else {
			if parts[0] == "dir" {
				newDir := NewDir(parts[1], currentNode)
				(*currentNode).AddChild(&newDir)
			} else {
				size, err := strconv.Atoi(parts[0])
				if err != nil {
					return 0, err
				}
				newFile := NewFile(parts[1], size, currentNode)
				(*currentNode).AddChild(&newFile)
			}
		}
	}
	LogTree(root_node)
	return CalculateResult(root_node), nil
}

func CalculateResult(node Node) int {
	return calculateResult(node, 0)
}

func calculateResult(node Node, result int) int {
	if node.IsDir() && node.Size() < 100000 {
		result += node.Size()
	}
	for _, child := range node.Children() {
		result = calculateResult(*child, result)
	}
	return result
}

func LogTree(node Node) {
	logTree(node, "")
}

func logTree(node Node, ident string) {
	if (node).IsDir() {
		fmt.Println(ident, node.Name(), "dir", node.Size())
	} else {
		fmt.Println(ident, node.Name(), node.Size())
	}
	for _, child := range node.Children() {
		logTree(*child, fmt.Sprintf("%s  ", ident))
	}
}

func findChild(node Node, name string) *Node {
	for _, child := range node.Children() {
		if (*child).Name() == name {
			return child
		}
	}
	return nil
}

type Node interface {
	Name() string
	IsDir() bool
	Size() int
	Parent() *Node
	Children() []*Node
	AddChild(*Node)
}

type DirNode struct {
	name     string
	parent   *Node
	children []*Node
}

func (n *DirNode) Name() string {
	return n.name
}

func (n *DirNode) IsDir() bool {
	return true
}

func (n *DirNode) Size() int {
	size := 0
	for _, child := range n.Children() {
		size += (*child).Size()
	}
	return size
}

func (n *DirNode) Parent() *Node {
	return n.parent
}

func (n *DirNode) Children() []*Node {
	return n.children
}

func (n *DirNode) AddChild(node *Node) {
	n.children = append(n.children, node)
}

func NewDir(name string, parent *Node) Node {
	return &DirNode{name: name, parent: parent}
}

type FileNode struct {
	name   string
	size   int
	parent *Node
}

func (n *FileNode) Name() string {
	return n.name
}

func (n *FileNode) IsDir() bool {
	return false
}

func (n *FileNode) Size() int {
	return n.size
}

func (n *FileNode) Parent() *Node {
	return n.parent
}

func (n *FileNode) Children() []*Node {
	return []*Node{}
}

func (n *FileNode) AddChild(node *Node) {
	panic(fmt.Sprintf("Cannot add child (%s) to file (%s)", (*node).Name(), n.name))
}

func NewFile(name string, size int, parent *Node) Node {
	return &FileNode{name: name, size: size, parent: parent}
}
