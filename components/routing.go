package components

import (
	"errors"
	"fmt"
)

func (a *Application) HandleInboundRequest(discordRequest DiscordRequest) (*DiscordResponse, error) {
	interactionType := discordRequest.ResponseType
	data := discordRequest.Data
	if interactionType == 1 {
		response := PingResponse()
		return &response, nil
	}
	if interactionType != 2 {
		return nil, errors.New(fmt.Sprintf("unsupported request type: %v", interactionType))
	}
	switch data.Name {
	case "bet":
		return a.HandleInboundRequestPlaceWager(data)
	case "bets":
		if discordRequest.Member == nil || discordRequest.Member.User == nil {
			return nil, errors.New(fmt.Sprintf("member or user not included in request"))
		}
		return a.HandleInboundRequestListWagers(discordRequest.Member.User)
	default:
		return nil, errors.New(fmt.Sprintf("unknown interaction name: %s", data.Name))
	}
}

func (a *Application) HandleInboundRequestPlaceWager(data *InteractionData) (*DiscordResponse, error) {
	wager, err := CreateWagerFromOptions(data.Options)
	if err != nil {
		return nil, err
	}
	err = a.repo.Insert(wager)
	if err != nil {
		return nil, err
	}
	message := fmt.Sprintf("wager placed: %s", wager.Summary())
	response := SimpleResponse(message)
	return &response, nil
}

func (a *Application) HandleInboundRequestListWagers(data *DiscordUser) (*DiscordResponse, error) {
	user := fmt.Sprintf("<@%s>", data.Id)
	wagers := a.repo.FindWagersForUser(user)
	if len(wagers) == 0 {
		response := SimpleResponse(fmt.Sprintf("%s has no outstanding wagers", user))
		return &response, nil
	}

	message := fmt.Sprintf("%s outstanding wagers:", user)
	for _, wager := range wagers {
		message += fmt.Sprintf("\n- %s", wager.Summary())
	}
	response := SimpleResponse(message)
	return &response, nil
}
