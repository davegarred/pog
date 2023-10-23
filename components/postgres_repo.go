package components

import (
	"database/sql"
	_ "github.com/lib/pq"
	"log"
)

const insertWager = "INSERT INTO wagers(wager_id,offering,accepting,wager,outcome,status) VALUES (nextval('seq_wager_id'), $1, $2, $3, $4, 0)"
const selectByUser = "SELECT * FROM wagers WHERE offering= $1 or accepting= $2"

type PostgresWagerRepository struct {
	db *sql.DB
}

func NewPostgresWagerRepository(connection string) (*PostgresWagerRepository, error) {
	db, err := sql.Open("postgres", connection)
	if err != nil {
		return nil, err
	}
	repository := PostgresWagerRepository{
		db: db,
	}
	return &repository, nil
}

func (r *PostgresWagerRepository) Insert(wager Wager) error {
	stmt, err := r.db.Prepare(insertWager)
	if err != nil {
		return err
	}
	defer stmt.Close()

	_, err = stmt.Exec(wager.Offering, wager.Accepting, wager.Wager, wager.Outcome)
	return err
}
func (r *PostgresWagerRepository) FindWagersForUser(user string) []Wager {
	result := make([]Wager, 0)

	stmt, err := r.db.Prepare(selectByUser)
	if err != nil {
		log.Fatal(err)
		return []Wager{}
	}
	defer stmt.Close()

	rows, err := stmt.Query(user, user)
	for rows.Next() {
		var (
			id, status                          int
			offering, accepting, wager, outcome string
		)
		if err := rows.Scan(&id, &offering, &accepting, &wager, &outcome, &status); err != nil {
			log.Fatal(err)
		}
		result = append(result, Wager{
			Offering:  offering,
			Accepting: accepting,
			Wager:     wager,
			Outcome:   outcome,
		})
	}
	return result
}
