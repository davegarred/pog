package components

import (
	"context"
	"crypto/ed25519"
	"encoding/hex"
	"encoding/json"
	"github.com/aws/aws-lambda-go/events"
	"log"
)

type Application struct {
	publicKey ed25519.PublicKey
	repo      WagerRepository
}

func NewApplication(publicKey string, repository WagerRepository) Application {
	keyBytes, err := hex.DecodeString(publicKey)
	if err != nil {
		panic("public key that was provided is not a valid hex value")
	}
	return Application{
		publicKey: keyBytes,
		repo:      repository,
	}
}

func (a *Application) HandleRequest(ctx context.Context, request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	log.Printf("body: %s\n", request.Body)
	if !VerifyRequest(&request, a.publicKey) {
		log.Println("verification failed")
		return notAuthorized(), nil
	}

	discordRequest := DiscordRequest{}
	err := json.Unmarshal([]byte(request.Body), &discordRequest)
	if err != nil {
		log.Printf("ERROR - unable to deserialize inbound request: %v", err)
		return badRequest(), nil
	}

	response, err := a.HandleInboundRequest(discordRequest)
	if err != nil {
		log.Printf("ERROR - unexpected error handling request: %v", err)
		return badRequest(), nil
	}
	return okay(response), nil
}

func okay(response *DiscordResponse) events.APIGatewayProxyResponse {
	responseBody, err := json.Marshal(response)
	if err != nil {
		panic(err)
	}
	return events.APIGatewayProxyResponse{
		StatusCode: 200,
		Body:       string(responseBody),
	}
}

func notAuthorized() events.APIGatewayProxyResponse {
	return events.APIGatewayProxyResponse{
		StatusCode: 401,
	}
}

func badRequest() events.APIGatewayProxyResponse {
	return events.APIGatewayProxyResponse{
		StatusCode: 400,
	}
}
