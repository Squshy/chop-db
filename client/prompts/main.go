package prompts

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

type PromptCode int
type State int

const (
	OutsideQuotes State = iota
	InsideQuotes
	Escape
)

const (
	EXIT PromptCode = iota
	GET
	SET
	DELETE
)

type InputData struct {
	Code   PromptCode
	Values []string
}

type GetCommand struct {
	Key string
}

type SetCommand struct {
	Key   string
	Value string
}

type DeleteCommand struct {
	Key string
}

func ParseGetCommand(input []string) (GetCommand, error) {
	if len(input) != 1 {
		return GetCommand{Key: ""}, errors.New("Invalid GET command: missing key")
	}

	return GetCommand{Key: input[0]}, nil
}

func ParseSetCommand(input []string) (SetCommand, error) {
	if len(input) != 2 {
		return SetCommand{Key: "", Value: ""}, errors.New("Invalid SET command: Missing key/value pair")
	}

	return SetCommand{Key: input[0], Value: input[1]}, nil
}

func ParseDeleteCommand(input []string) (DeleteCommand, error) {
	if len(input) != 1 {
		return DeleteCommand{Key: ""}, errors.New("Invalid DELETE command: missing key")
	}

	return DeleteCommand{Key: input[0]}, nil
}

func splitString(input string) ([]string, error) {
	state := OutsideQuotes
	var fields []string
	var currentField string

	for idx, char := range input {
		switch state {
		case OutsideQuotes:
			if unicode.IsSpace(char) {
				if currentField != "" {
					fields = append(fields, currentField)
					currentField = ""
				}
				continue
			}

			if char == '"' {
				state = InsideQuotes
			} else {
				currentField += string(char)
			}

		case InsideQuotes:
			// We are adding a `"` and the last character was not an escape
			if char == '"' && []rune(input)[len(input)-1] != '\\' {
				state = OutsideQuotes
			} else if char == '\\' && idx+1 < len(input) && []rune(input)[idx+1] == '"' {
				state = Escape
			} else {
				currentField += string(char)
			}

		case Escape:
			state = InsideQuotes
			currentField += string(char)
		}
	}

	if state == InsideQuotes {
		return []string{}, errors.New("Invalid value: Unfinished string")
	}

	if state == Escape {
		return []string{}, errors.New("Invalid value: Unescaped string")
	}

	if state == OutsideQuotes && currentField != "" {
		fields = append(fields, currentField)
	}

	return fields, nil
}

func validateInput(input string) (InputData, error) {
	// Don't really need to do this at the moment tbh.
	// I very well could just accept all input after the initial command/key
	// split as values since `set` is the only one that needs it.
	splits, err := splitString(input)

	if err != nil {
		return InputData{Code: -1, Values: []string{""}}, err
	}

	// Get the first word in the line
	switch strings.ToLower(splits[0]) {
	case "bye", "exit", "quit", "q":
		return InputData{Code: EXIT, Values: splits[1:]}, nil
	case "get", "g":
		return InputData{Code: GET, Values: splits[1:]}, nil
	case "set", "s":
		return InputData{Code: SET, Values: splits[1:]}, nil
	case "delete", "d", "del":
		return InputData{Code: DELETE, Values: splits[1:]}, nil
	}

	return InputData{Code: -1, Values: []string{""}}, errors.New("Invalid input: Unrecognized command " + splits[0])
}

func PromptForCommand() (InputData, error) {

	var input string
	scanner := bufio.NewScanner(os.Stdin)

	fmt.Fprint(os.Stderr, "chop:: ")
	for scanner.Scan() {
		input = scanner.Text()

		if input != "" {
			break
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatalf("Error: %v", err)
	}

	return validateInput(input)
}
