#!/bin/bash
# Starts up a local service under docker

set -u

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
source "${SCRIPT_DIR}/.env"
source "${SCRIPT_DIR}/common.sh"

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

SERVICE=${1}
ENVIRONMENT=${2}
CLIENT_LAMBDA=${3:-}
DB_HOST=${4:-}

case "$SERVICE" in
    client)
        NETWORKING_ARGS="-p 8080:80"
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

docker run -d \
  -e ENVIRONMENT="$ENVIRONMENT" \
  -e DISCORD_APPLICATION_ID="${DISCORD_APPLICATION_ID}" \
  -e APPLICATION_TOKEN="${DISCORD_TOKEN}" \
  -e DISCORD_TOKEN="${DISCORD_TOKEN}" \
  -e DISCORD_PUBLIC_KEY="${DISCORD_PUBLIC_KEY}" \
  -e GEMINI_TOKEN="${GEMINI_TOKEN}" \
  -e CLIENT_LAMBDA="${CLIENT_LAMBDA}" \
  -e DB_USER="${DB_USER}" \
  -e DB_PASS="${DB_PASS}" \
  -e DB_HOST="${DB_HOST}" \
  -e DB_NAME="${DB_NAME}" \
  "${NETWORKING_ARGS:-}" \
  ${SERVICE}
