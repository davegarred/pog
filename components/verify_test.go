package components

import (
	"bytes"
	"crypto/ed25519"
	"encoding/hex"
	"github.com/aws/aws-lambda-go/events"
	"strconv"
	"testing"
	"time"
)

const timestamp = "1608597133"
const body = `{"id":"my-object-id","name":"an-interaction-name"}`

var (
	pubkey, privkey, _ = ed25519.GenerateKey(nil)
	headerSignature    = hex.EncodeToString(signature[:ed25519.SignatureSize])
	signature          = buildSig(timestamp, body)
)

func TestVerifyInteraction(t *testing.T) {

	t.Run("success", func(t *testing.T) {
		request := events.APIGatewayProxyRequest{
			Headers: map[string]string{
				"x-signature-timestamp": timestamp,
				"x-signature-ed25519":   headerSignature,
			},
			Body: body,
		}
		if !VerifyRequest(&request, pubkey) {
			t.Error("failed to verify valid request")
		}
	})

	t.Run("modified body", func(t *testing.T) {
		request := events.APIGatewayProxyRequest{
			Headers: map[string]string{
				"x-signature-timestamp": timestamp,
				"x-signature-ed25519":   headerSignature,
			},
			Body: "This is not the correct body",
		}

		if VerifyRequest(&request, pubkey) {
			t.Error("modified body was not caught")
		}
	})

	t.Run("modified timestamp", func(t *testing.T) {
		request := events.APIGatewayProxyRequest{
			Headers: map[string]string{
				"x-signature-timestamp": strconv.FormatInt(time.Now().Add(time.Minute).Unix(), 10),
				"x-signature-ed25519":   headerSignature,
			},
			Body: body,
		}

		if VerifyRequest(&request, pubkey) {
			t.Error("modified timestamp was not caught")
		}
	})
}

func buildSig(timestamp, body string) []byte {
	var msg bytes.Buffer
	msg.WriteString(timestamp)
	msg.WriteString(body)
	return ed25519.Sign(privkey, msg.Bytes())
}
