package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"unicode/utf8"

	"github.com/fatih/color"
)

const defaultPath = "~/Documents/Projects"

var crossedOut = color.New(color.CrossedOut)

type Project struct {
	Name        string
	Path        string
	IsCommented bool
}
type Options struct {
	CustomPath bool
	Clean      bool
	List       bool
	Remove     bool
	RemoveF    bool
	Update     bool
}

func main() {
	customPath := flag.Bool("c", false, "Custom path to add project")
	clean := flag.Bool("clean", false, "Clear commented projects from the file")
	list := flag.Bool("l", false, "ListProjects current projects and paths")
	remove := flag.Bool("r", false, "Remove project")
	removeF := flag.Bool("rf", false, "Force remove project")
	update := flag.Bool("U", false, "Update project list in ~/Documents/Projects")
	flag.Parse()

	args := flag.Args()
	if (len(args) == 0 && !*clean) && !*list && !*update {
		flag.CommandLine.Usage()
		Err("", 0)
	}

	// check viable args
	CheckArgs(Options{*customPath, *clean, *list, *remove, *removeF, *update})
	// read current zshalias file
	homeDir, err := os.UserHomeDir()
	Check(err)
	path := fmt.Sprintf("%v/.zshalias", homeDir)
	b, err := os.ReadFile(path)
	Check(err)
	lines := strings.Split(string(b), "\n")[1:]
	projects := make([]*Project, 0)

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
			projects = append(projects, &proj)
		}
	}

	if *list {
		ListProjects(projects)
		os.Exit(0)
	}

	if *update {
		// get all paths in ~/Documents/Projects
		projsDir, err := os.ReadDir(homeDir + "/Documents/Projects")
		Check(err)
		for _, v := range projsDir {
			found := false
			projPath := "~/Documents/Projects/" + v.Name()
			for _, p := range projects {

				p.Path = strings.ReplaceAll(p.Path, "/Users/rrossmil", "~")
				if p.Path[utf8.RuneCountInString(p.Path)-1] == '/' {
					p.Path = p.Path[:utf8.RuneCountInString(p.Path)-1]
				}
				// if the paths match, use the name from the zshalias file
				if projPath == p.Path {
					found = true
				}
			}
			// if the project wasn't already found, add it to the list of projects
			if !found {
				projects = append(projects, &Project{v.Name(), projPath, false})
			}
		}
	}

	// add project from user input
	if !*update {
		if *remove || *removeF {
			projects = RemoveProject(args[0], projects, *removeF)
		} else if *clean {
			projects = Clean(projects)
		} else {
			var path string
			if *customPath {
				path = args[1]
			} else {
				path = defaultPath
			}
			projects = AddProject(args[0], path, projects)
		}
	}
	// write updated file
	WriteAliasProjectFile(path, projects)
	Finish()
}

func GetProjectFromLine(i int, line string) Project {
	splits := strings.Split(line, "=")
	if len(splits) < 2 {
		Err(fmt.Sprintf("Line %d is malformed. Should contain '='", i+1), 0)
	}
	// get name
	alias := strings.Split(splits[0], " ")
	if alias[0] != "alias" {
		Err(fmt.Sprintf("Line %d is malformed. Should start with alias", i+1), 0)
	}
	name := alias[1][2:]

	// get path
	pSplit := strings.Split(splits[1], "&&")[0]
	pSplit = strings.Split(pSplit, " ")[1]
	return Project{name, pSplit, false}
}

func RemoveProject(name string, projects []*Project, force bool) []*Project {
	rtn := make([]*Project, 0)
	if name[0] == '#' {
		Err("Project name must not start with '#'", 0)
	}

	foundName := false
	for _, p := range projects {
		if !force && name == p.Name {
			foundName = true
			p.IsCommented = true
			rtn = append(rtn, p)
		} else if force && name == p.Name {
			foundName = true
		} else {
			rtn = append(rtn, p)
		}
	}
	if !foundName {
		Err(fmt.Sprintf("Did not find project '%v'", name), 0)
	}
	return rtn
}

func Clean(projects []*Project) []*Project {
	rtn := make([]*Project, 0)
	for _, p := range projects {
		if !p.IsCommented {
			rtn = append(rtn, p)
		}
	}
	return rtn
}

func AddProject(name, path string, projects []*Project) []*Project {
	if name[0] == '#' {
		Err("Project name must not start with '#'", 0)
	}
	// if name is in projects, err
	for i, p := range projects {
		if name == p.Name && !p.IsCommented {
			message := fmt.Sprintf("'%v' already exists", name)
			Err(message, 0)
		} else if name == p.Name && p.IsCommented {
			// if name is commented out and paths match, uncomment
			projects[i].IsCommented = false
			return projects
		}
	}

	path = path + "/" + name
	return append(projects, &Project{name, path, false})
}

func WriteAliasProjectFile(path string, projects []*Project) {
	f, err := os.Create(path)
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

func ListProjects(projects []*Project) {
	for _, p := range projects {
		s := fmt.Sprintf("%v - %v\n", p.Name, p.Path)
		if p.IsCommented {
			crossedOut.Print(s)
		} else {
			fmt.Print(s)
		}
	}
}

// type Options struct {
// 	CustomPath bool
// 	Clean      bool
// 	List       bool
// 	Remove     bool
// 	RemoveF    bool
// 	Update     bool

// customPath := flag.Bool("c", false, "Custom path to add project")
// clean := flag.Bool("clean", false, "Clear commented projects from the file")
// list := flag.Bool("l", false, "ListProjects current projects and paths")
// remove := flag.Bool("r", false, "Remove project")
// removeF := flag.Bool("rf", false, "Force remove project")
// update := flag.Bool("U", false, "Update project list in ~/Documents/Projects")
// }

func CheckArgs(opts Options) {
	switch {
	case opts.List && (opts.CustomPath || opts.Clean || opts.Remove || opts.RemoveF || opts.Update):
		Err("Cannot list projects and do any other action", 0)
		// case TODO others
	}
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func Err(message string, code int) {
	fmt.Println(message)
	os.Exit(code)
}

func Finish() {
	exec.Command("rzshrc").Output()
}
