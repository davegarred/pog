#!/bin/bash
# Deploys a Cloud Run instance on GCP running the server image (tag: latest)

set -u

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
source "${SCRIPT_DIR}/.env"
source "${SCRIPT_DIR}/common.sh"

if [[ -z "${POG_REPO:-}" ]];then
    echo "expects a repository in environment variable: POG_REPO"
    exit 1
fi

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME="pog-server-${ENVIRONMENT}-${DEPLOYMENT_TIME}"
IMAGE_NAME="${POG_REPO}/pog_server:latest"

CLIENT_IP_ADDRESS=$( gcloud compute instances list | grep "pog-client-${ENVIRONMENT}" | tail -1 | awk '{print $4}' )
if [[ ${CLIENT_IP_ADDRESS} == "" ]];then
  echo "Unable to locate a client instance using:"
  echo "  gcloud compute instances list | grep pog-client-${ENVIRONMENT}"
  exit 1
fi
CLIENT_LAMBDA="http://${CLIENT_IP_ADDRESS}"
echo "identified client url as: ${CLIENT_LAMBDA}"

gcloud run deploy "$INSTANCE_NAME" \
  --image="$IMAGE_NAME" \
  --allow-unauthenticated \
  --port=8080 \
  --network=personal-vpc \
  --subnet=personal-vpc-central1 \
  --vpc-egress=private-ranges-only \
  --set-env-vars=CLIENT_LAMBDA="${CLIENT_LAMBDA}" \
  --set-env-vars=DB_HOST="${DB_HOST}" \
  --set-env-vars=DB_NAME="${DB_NAME}" \
  --set-env-vars=DB_USER="${DB_USER}" \
  --set-env-vars=DB_PASS="${DB_PASS}" \
  --set-env-vars=DISCORD_PUBLIC_KEY="${DISCORD_PUBLIC_KEY}" \
  --set-env-vars=DISCORD_APPLICATION_ID="${DISCORD_APPLICATION_ID}" \
  --set-env-vars=DISCORD_TOKEN="${DISCORD_TOKEN}" \
  --set-env-vars=ENVIRONMENT="${ENVIRONMENT}" \
  --set-env-vars=RUST_BACKTRACE=1 \
  --no-cpu-throttling \
  --set-cloudsql-instances=personal-cloud-352317:us-central1:garred-personal \
  --region=us-central1
