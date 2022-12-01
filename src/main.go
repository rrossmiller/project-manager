package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strings"
	"unicode/utf8"
	// "github.com/fatih/color"
)

const defaultPath = "~/Documents/Projects"

// var red = color.New(color.FgRed).SprintFunc()
// var blue = color.New(color.FgBlue).SprintFunc()

type Project struct {
	Name        string
	Path        string
	IsCommented bool
}

func main() {
	customPath := flag.Bool("c", false, "Custom path to add project")
	remove := flag.Bool("r", false, "Remove project")
	flag.Parse()

	args := flag.Args()
	if len(args) == 0 {
		flag.CommandLine.Usage()
		Err("", 0)
	}

	// read current zshalias file
	b, err := os.ReadFile("../.zshalias")
	Check(err)
	lines := strings.Split(string(b), "\n")[1:]
	projects := make([]Project, 0)

	for i, line := range lines {
		if utf8.RuneCountInString(line) > 0 {
			isCommented := false
			if line[0] == '#' {
				isCommented = true
				line = line[1:]
			}

			line = strings.TrimSpace(line)

			proj := GetProjectFromLine(i, line)
			proj.IsCommented = isCommented
			projects = append(projects, proj)
		}
	}

	fmt.Println(projects)

	// add project from user input
	if *remove {
		fmt.Println("remove")
	} else {
		var path string
		if *customPath {
			path = args[1]
		} else {
			path = defaultPath
		}
		projects = AddProject(args[0], path, projects)
	}
	// write updated file
	WriteAliasProjectFile(projects)
}

func GetProjectFromLine(i int, line string) Project {
	splits := strings.Split(line, "=")
	if len(splits) < 2 {
		Err(fmt.Sprintf("Line %d is malformed. Should contain '='\n", i+1), 0)
	}
	// get name
	alias := strings.Split(splits[0], " ")
	if alias[0] != "alias" {
		Err(fmt.Sprintf("Line %d is malformed. Should start with alias\n", i+1), 0)
	}
	name := alias[1][2:]

	// get path
	pSplit := strings.Split(splits[1], "&&")[0]
	pSplit = strings.Split(pSplit, " ")[1]
	return Project{name, pSplit, false}
}

func AddProject(name, path string, projects []Project) []Project {
	if name[0] == '#' {
		Err("Project name must not start with '#'\n", 0)
	}
	path = path + "/" + name
	return append(projects, Project{name, path, false})
}

func WriteAliasProjectFile(projects []Project) {
	f, err := os.Create("../.zshalias")
	Check(err)
	defer f.Close()
	w := bufio.NewWriter(f)
	_, err = w.WriteString("# Project paths\n\n")
	Check(err)
	for _, p := range projects {
		line := fmt.Sprintf("alias pp%v='cd %v && clear && work'\n", p.Name, p.Path)

		if p.IsCommented {
			line = "# " + line
		}
		w.WriteString(line)
	}
	w.Flush()
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func Err(message string, code int) {
	fmt.Print(message)
	os.Exit(code)
}
