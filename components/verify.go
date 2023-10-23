package components

import (
	"bytes"
	"crypto/ed25519"
	"encoding/hex"
	"github.com/aws/aws-lambda-go/events"
	"strings"

	"io"
)

func VerifyRequest(request *events.APIGatewayProxyRequest, key ed25519.PublicKey) bool {
	if request.Headers["x-signature-ed25519"] == "" || request.Headers["x-signature-timestamp"] == "" {
		return false
	}
	sig, err := hex.DecodeString(request.Headers["x-signature-ed25519"])
	if err != nil || len(sig) != ed25519.SignatureSize || sig[63]&224 != 0 {
		return false
	}

	var msg bytes.Buffer
	msg.WriteString(request.Headers["x-signature-timestamp"])
	_, err = io.Copy(&msg, strings.NewReader(request.Body))
	if err != nil {
		return false
	}

	return ed25519.Verify(key, msg.Bytes(), sig)
}
