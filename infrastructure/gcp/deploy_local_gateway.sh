#!/bin/bash
# Starts up a local gateway server

function usage() {
  cat >&2 << USAGE
usage:
  $0 <environment>

USAGE
}

if [[ $# -ne 1 ]];
then
    usage
    exit 1
fi


ENVIRONMENT=$1
DISCORD_APP_ID_KEY=pog-discord_app_id-$ENVIRONMENT
DISCORD_APP_TOKEN_KEY=pog-discord_app_token-$ENVIRONMENT

if ! DISCORD_APP_ID=$( gcloud secrets versions access latest --secret="$DISCORD_APP_ID_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi
if ! DISCORD_APP_TOKEN=$( gcloud secrets versions access latest --secret="$DISCORD_APP_TOKEN_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi
GEMINI_TOKEN="__GeminiToken__"
CLIENT_LAMBDA="__PogClientLambda__"

docker run --rm -d \
    -e DISCORD_APPLICATION_ID="$DISCORD_APP_ID" \
    -e APPLICATION_TOKEN="$DISCORD_APP_TOKEN" \
    -e GEMINI_TOKEN="$GEMINI_TOKEN" \
    -e CLIENT_LAMBDA="$CLIENT_LAMBDA" \
    gateway