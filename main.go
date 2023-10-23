package main

import (
	"os"
	"pog/components"
)

import (
	"github.com/aws/aws-lambda-go/lambda"
)

func main() {
	connection := os.Getenv("DB_CONNECTION_STRING")
	publicKey := os.Getenv("DISCORD_PUBLIC_KEY")
	repo, err := components.NewPostgresWagerRepository(connection)
	if err != nil {
		panic(err)
	}
	application := components.NewApplication(publicKey, repo)
	lambda.Start(application.HandleRequest)
}
