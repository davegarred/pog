package components

type WagerRepository interface {
	Insert(Wager) error
	FindWagersForUser(user string) []Wager
}

type InMemWagerRepo struct {
	wagers []Wager
}

func NewInMemWagerRepo() InMemWagerRepo {
	return InMemWagerRepo{
		wagers: make([]Wager, 0),
	}
}

func (r *InMemWagerRepo) Insert(wager Wager) error {
	r.wagers = append(r.wagers, wager)
	return nil
}

func (r *InMemWagerRepo) FindWagersForUser(user string) []Wager {
	result := []Wager{}
	for _, wager := range r.wagers {
		if wager.Offering == user || wager.Accepting == user {
			result = append(result, wager)
		}
	}
	return result
}
