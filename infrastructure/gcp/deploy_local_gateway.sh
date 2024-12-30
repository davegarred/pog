#!/bin/bash
# Starts up a local service under docker

function usage() {
  cat >&2 << USAGE
usage:
  $0 <service> <environment> [client_url] [database_ip]
      service       One of: server, client, gateway
      environment   One of: dev, prod

USAGE
}

if [[ $# -lt 2 ]];
then
    usage
    exit 1
fi

SERVICE=$1
ENVIRONMENT=$2
CLIENT_LAMBDA=$3
DB_HOST=$4

DB_NAME="pog_server"
DB_USER="pog_user"
DB_PASS="pog_pass"

DISCORD_APP_ID_KEY=pog-discord_app_id-$ENVIRONMENT
DISCORD_APP_TOKEN_KEY=pog-discord_app_token-$ENVIRONMENT
DISCORD_PUBLIC_KEY_KEY=pog-public_key-$ENVIRONMENT
GEMINI_TOKEN_KEY=pog-gemini_token-$ENVIRONMENT

case "$SERVICE" in
    client)
        NETWORKING_ARGS="-p 8080:8080"
        SERVICE=pog_client
        ;;
    server)
        NETWORKING_ARGS="-p 8090:8080"
        SERVICE=pog_server
        ;;
    gateway)
        SERVICE=pog_gateway
        ;;
    *)
        usage
        exit 1
        ;;
esac


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
if ! DISCORD_PUBLIC_KEY=$( gcloud secrets versions access latest --secret="$DISCORD_PUBLIC_KEY_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi
if ! GEMINI_TOKEN=$( gcloud secrets versions access latest --secret="$GEMINI_TOKEN_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi

docker run --rm -d \
    -e ENVIRONMENT="$ENVIRONMENT" \
    -e DISCORD_APPLICATION_ID="$DISCORD_APP_ID" \
    -e APPLICATION_TOKEN="$DISCORD_APP_TOKEN" \
    -e DISCORD_TOKEN="$DISCORD_APP_TOKEN" \
    -e DISCORD_PUBLIC_KEY="$DISCORD_PUBLIC_KEY" \
    -e GEMINI_TOKEN="$GEMINI_TOKEN" \
    -e CLIENT_LAMBDA="$CLIENT_LAMBDA" \
    -e DB_USER="$DB_USER" \
    -e DB_PASS="$DB_PASS" \
    -e DB_HOST="$DB_HOST" \
    -e DB_NAME="$DB_NAME" \
    $NETWORKING_ARGS \
    $SERVICE
