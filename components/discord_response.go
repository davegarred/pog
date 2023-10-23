package components

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object
type DiscordResponse struct {
	ResponseType uint8                    `json:"type"`
	Data         *InteractionCallbackData `json:"data"`
}

func PingResponse() DiscordResponse {
	return DiscordResponse{
		ResponseType: 1,
	}
}

func SimpleResponse(message string) DiscordResponse {
	return DiscordResponse{
		ResponseType: 4,
		Data: &InteractionCallbackData{
			Content: message,
		},
	}
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure
type InteractionCallbackData struct {
	Content string `json:"content"`
}
