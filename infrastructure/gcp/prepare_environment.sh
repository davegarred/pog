#!/bin/bash
# Downloads and environment variables

set -u

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
ENVIRONMENT_VAR_STORE="${SCRIPT_DIR}/.env"

if [[ $# -ne 1 ]];
then
    echo "usage:"
    echo "  $0 <environment>"
    exit 1
fi

function get_secret {
  SECRET_KEY=$1
  ENV_VAR=$2
  if ! SECRET_VALUE=$( gcloud secrets versions access latest --secret="$SECRET_KEY" )
  then
      echo "expected secret was not found: $1"
      exit 1
  fi
  echo "${ENV_VAR}=${SECRET_VALUE}" >> "${ENVIRONMENT_VAR_STORE}"
}

ENVIRONMENT=$1
DISCORD_APP_ID_KEY=pog-discord_app_id-$ENVIRONMENT
DISCORD_APP_TOKEN_KEY=pog-discord_app_token-$ENVIRONMENT
DISCORD_PUBLIC_KEY_KEY=pog-public_key-$ENVIRONMENT
DB_USER_KEY=pog-db_user-$ENVIRONMENT
DB_PASS_KEY=pog-db_pass-$ENVIRONMENT
DB_HOST_KEY=pog-db_host-$ENVIRONMENT
DB_NAME_KEY=pog-db_name-$ENVIRONMENT
GEMINI_TOKEN_KEY=pog-gemini_token-$ENVIRONMENT

echo "ENVIRONMENT=${ENVIRONMENT}" > "${ENVIRONMENT_VAR_STORE}"
get_secret "${DISCORD_APP_ID_KEY}" "DISCORD_APPLICATION_ID"
get_secret "${DISCORD_APP_TOKEN_KEY}" "DISCORD_TOKEN"
get_secret "${DISCORD_PUBLIC_KEY_KEY}" "DISCORD_PUBLIC_KEY"
get_secret "${DB_USER_KEY}" "DB_USER"
get_secret "${DB_PASS_KEY}" "DB_PASS"
get_secret "${DB_HOST_KEY}" "DB_HOST"
get_secret "${DB_NAME_KEY}" "DB_NAME"
get_secret "${GEMINI_TOKEN_KEY}" "GEMINI_TOKEN"
