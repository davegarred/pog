package components

import (
	"reflect"
	"testing"
)

func TestNewInMemWagerRepo(t *testing.T) {
	repo := NewInMemWagerRepo()
	wagerA := Wager{
		Offering:  "<@695398918694895710>",
		Accepting: "Woody",
		Wager:     "$20",
		Outcome:   "Raiders win over Oilers",
	}
	wagerB := Wager{
		Offering:  "Woody",
		Accepting: "Rick",
		Wager:     "+500 pays $100/loses $20",
		Outcome:   "No hitter thrown in ALCS game 6",
	}
	repo.Insert(wagerA)
	repo.Insert(wagerB)
	repo.Insert(Wager{
		Offering:  "<@695398918694895710>",
		Accepting: "Rick",
		Wager:     "$40",
		Outcome:   "Dallas in the playoffs",
	})
	result := repo.FindWagersForUser("Woody")
	expected := []Wager{wagerA, wagerB}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected: %+v\nFound: %+v\v", expected, result)
	}
}
