# Project Manager


# Rust
```
source ~/.project-aliases
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
