package components

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"reflect"
	"testing"
)

//  TODO: CHECK THAT ORIGINAL DATA PRODUCES THE SAME AS A SERIALIZATION
func TestPingRequest(t *testing.T) {
	request := loadDiscordRequest("../dto_payloads/ping_request.json")
	if err := validateDiscordRequest(request); err != nil {
		t.Errorf("dto validation failed: %v\n", err)
	}
}

func TestLargeRequest(t *testing.T) {
	request := loadDiscordRequest("../dto_payloads/request.json")
	if err := validateDiscordRequest(request); err != nil {
		t.Errorf("dto validation failed: %v\n", err)
	}
}

func loadDiscordRequest(filename string) DiscordRequest {
	request := DiscordRequest{}
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	data, err := io.ReadAll(file)
	if err != nil {
		panic(err)
	}
	if err = json.Unmarshal(data, &request); err != nil {
		panic(err)
	}
	return request
}

func validateDiscordRequest(dto interface{}) error {
	deserialized := DiscordRequest{}
	ser, _ := json.Marshal(dto)
	fmt.Printf("%s\n", string(ser))
	json.Unmarshal(ser, &deserialized)
	if !reflect.DeepEqual(deserialized, dto) {
		message := fmt.Sprintf("not the same: \noriginal - %+v\n found  - %+v\n", dto, deserialized)
		return errors.New(message)
	}
	return nil
}
