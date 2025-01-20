#!/bin/bash
# Deploys a Cloud Run instance on GCP running the client image (tag: latest)

set -u

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
source "${SCRIPT_DIR}/.env"

if [[ -z "${POG_REPO:-}" ]];
then
    echo "expects a repository in environment variable: POG_REPO"
    exit 1
fi

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME="pog-client-${ENVIRONMENT}-${DEPLOYMENT_TIME}"
IMAGE_NAME="${POG_REPO}/pog_client:latest"

gcloud run deploy "${INSTANCE_NAME}" \
--image="${IMAGE_NAME}" \
--allow-unauthenticated \
--port=8080 \
--network=personal-vpc \
--subnet=personal-vpc-central1 \
--vpc-egress=private-ranges-only \
--ingress=internal \
--execution-environment=gen1 \
--set-env-vars=RUST_BACKTRACE=1 \
--region=us-central1
