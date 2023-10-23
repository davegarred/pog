package components

import (
	"encoding/json"
	"fmt"
	"reflect"
	"testing"
)

func TestSimpleResponse(t *testing.T) {
	response := SimpleResponse("looks like this is working")
	ser, _ := json.Marshal(response)
	fmt.Printf(string(ser))
	deserialized := DiscordResponse{}
	json.Unmarshal(ser, &deserialized)
	if !reflect.DeepEqual(deserialized, response) {
		t.Errorf("not the same: %s", string(ser))
	}
}
