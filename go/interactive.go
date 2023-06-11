package main

import (
	"fmt"
	"os"

	"github.com/rrossmiller/gocliselect"
)

func interactive() {
	menu := gocliselect.NewMenu("Chose a color")
	menu.VimKeys = true

	menu.AddItem("Red", "red")
	menu.AddItem("Blue", "blue")
	menu.AddItem("Green", "green")
	menu.AddItem("Yellow", "yellow")
	menu.AddItem("Cyan", "cyan")

	choice := menu.Display()

	fmt.Printf("Choice: %s\n", choice)
	os.Exit(0)
}
