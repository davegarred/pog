package components

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
type DiscordRequest struct {
	Id           string           `json:"id"`
	ResponseType uint8            `json:"type"`
	Data         *InteractionData `json:"data"`
	Member       *DiscordMember   `json:"member"`
	User         *DiscordUser     `json:"user"`
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data
type InteractionData struct {
	Id              string              `json:"id"`
	Name            string              `json:"name"`
	Options         []InteractionOption `json:"options"`
	InteractionType uint8               `json:"type"`
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-interaction-data-option-structure
type InteractionOption struct {
	Name    string              `json:"name"`
	Type    uint8               `json:"type"`
	Value   string              `json:"value"`
	Options []InteractionOption `json:"options"`
}

// https://discord.com/developers/docs/resources/guild#guild-member-object
type DiscordMember struct {
	User *DiscordUser `json:"user"`
}

// https://discord.com/developers/docs/resources/user#user-object
type DiscordUser struct {
	Id       string `json:"id"`
	UserName string `json:"username"`
	Avatar   string `json:"avatar"`
}
