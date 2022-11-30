package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
	"unicode/utf8"
	// "github.com/fatih/color"
)

const defaultPath = "~/Documents"

// var red = color.New(color.FgRed).SprintFunc()
// var blue = color.New(color.FgBlue).SprintFunc()

type Project struct {
	Name string
	Path string
}

func main() {
	// customPath := flag.Bool("c", false, "Custom path to add project")
	// remove := flag.Bool("r", false, "remove project")
	// regex := flag.Bool("r", false, "regex")
	flag.Parse()
	// args := flag.Args()

	// read current zshalias file
	b, err := os.ReadFile("../.zshalias")
	check(err)
	lines := string(b)
	projects := make([]Project, 0)

	for i, line := range strings.Split(lines, "\n") {
		if utf8.RuneCountInString(line) > 0 {
			line = strings.TrimSpace(line)
			if line[0] != '#' {
				proj := getProjectFromLine(i, line)
				projects = append(projects, proj)
			}
		}
	}

	fmt.Println(projects)

	// write updated file
}

func getProjectFromLine(i int, line string) Project {
	splits := strings.Split(line, "=")
	if len(splits) < 2 {
		fmt.Printf("Line %d is malformed. Should contain '='\n", i+1)
		os.Exit(0)
	}
	// get name
	alias := strings.Split(splits[0], " ")
	if alias[0] != "alias" {
		fmt.Printf("Line %d is malformed. Should start with alias\n", i+1)
		os.Exit(0)
	}
	name := alias[1][2:]

	// get path
	pSplit := strings.Split(splits[1], "&&")[0]
	pSplit = strings.Split(pSplit, " ")[1]
	return Project{name, pSplit}
}

func check(e error) {
	if e != nil {
		panic(e)
		// log.Fatal("something went wrong reading the fileNames")
	}
}
