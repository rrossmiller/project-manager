# Project Manager

Add the `work` function and `source ~/.project-aliases` to your .zshrc 

# Rust
```
source ~/.project-aliases
pm -l true
pp<alias name> #Cd's to the directory and calls the `work` function in your .zshrc (see bottom of the readme)
```

List projects
```
./pm list
./pm -l
./pm -l true # different formatting
```

Add project
```
./pm add project-name <path/to/project>
./pm -a project-name <path/to/project>
```

Toggle project
```
./pm toggle project-name 
./pm -t project-name 
```
Remove project
```
./pm delete project-name 
./pm -d project-name 
```


# Go
```
source ~/.zshalias
```

add or remove projects from zshalias

Add project using default path

```
./pm project-name <path/to/project>
```

Add project using custom path

```
./pm -c project-name path/to/project
```

Remove project

```
./pm -r project-name <path/to/project>
```

don't allow same project names


# The `work` function
put this in your .zshrc
```
function work() {
	clear
	echo Branches:
	git branch
	echo
	echo Contents:
	ls
	echo
	git status
	echo
}
```
