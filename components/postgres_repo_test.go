package components

import (
	"os"
	"reflect"
	"testing"
)

func TestPostgresWagerRepository(t *testing.T) {
	if os.Getenv("INTEGRATION-TESTING") == "" {
		t.Skip("not an integration test")
	}
	repo, err := NewPostgresWagerRepository("postgres://pog_user:pog_pass@localhost/pog?sslmode=disable")
	if err != nil {
		panic(err)
	}
	wager := Wager{
		Offering:  "Me",
		Accepting: "Woody",
		Wager:     "$20",
		Outcome:   "Raiders finish below 5-12",
	}
	err = repo.Insert(wager)
	if err != nil {
		panic(err)
	}
	result := repo.FindWagersForUser("Woody")
	expected := []Wager{wager}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected: %+v\nFound: %+v\v", expected, result)
	}
}
