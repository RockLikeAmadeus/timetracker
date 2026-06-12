package main

import (
	"fmt"
	"os"

	tea "charm.land/bubbletea/v2"
)

func main() {
	p := tea.NewProgram(initialState())
	if _, err := p.Run(); err != nil {
		fmt.Println("Uh oh: %v", err)
		os.Exit(1)
	}
}

type state struct {
	output string
}

func (s state) Init() tea.Cmd {
	return nil
}

func (s state) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyPressMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			fmt.Println("Bye!")
			return s, tea.Quit
		}
	}
	return s, nil
}

func (s state) View() tea.View {
	v := tea.NewView("test")
	return v
}

func initialState() state {
	return state{
		output: "Hello World",
	}
}