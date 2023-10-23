package components

import (
	"errors"
	"fmt"
)

type Wager struct {
	// TODO: add an ID and status
	Offering  string
	Accepting string
	Wager     string
	Outcome   string
}

func (w *Wager) Summary() string {
	return fmt.Sprintf("%s vs %s, %s - %s", w.Offering, w.Accepting, w.Wager, w.Outcome)
}

func CreateWagerFromOptions(options []InteractionOption) (Wager, error) {
	result := Wager{}
	for _, option := range options {
		switch option.Name {
		case "offering":
			result.Offering = option.Value
		case "accepting":
			result.Accepting = option.Value
		case "wager":
			result.Wager = option.Value
		case "outcome":
			result.Outcome = option.Value
		default:
			return result, errors.New(fmt.Sprintf("unknown interaction option encountered: %s - %s\n", option.Name, option.Value))
		}
	}

	return result, nil
}
