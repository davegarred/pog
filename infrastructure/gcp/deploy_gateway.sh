#!/bin/bash
# Deploys a VM on GCP running the gateway image (tag: latest)

set -u

SCRIPT_DIR="$(dirname "$(readlink   -f "$0")")"
source "${SCRIPT_DIR}/.env"
source "${SCRIPT_DIR}/common.sh"

if [[ -z "${POG_REPO:-}" ]];
then
    echo "expects a repository in environment variable: POG_REPO"
    exit 1
fi

CONTAINER_ENVS="RUST_BACKTRACE=1,GEMINI_TOKEN=${GEMINI_TOKEN},DISCORD_APPLICATION_ID=${DISCORD_APPLICATION_ID},APPLICATION_TOKEN=${DISCORD_TOKEN},ENVIRONMENT=${ENVIRONMENT},DB_HOST=${DB_HOST},DB_NAME=${DB_NAME},DB_USER=${DB_USER},DB_PASS=${DB_PASS}"

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME="pog-gateway-${ENVIRONMENT}-${DEPLOYMENT_TIME}"
IMAGE_NAME="${POG_REPO}/pog_gateway:latest"

gcloud compute instances create-with-container "${INSTANCE_NAME}" \
  --zone=us-central1-c \
  --machine-type=e2-micro \
  --network-interface=network-tier=PREMIUM,stack-type=IPV4_ONLY,subnet=personal-vpc-central1 \
  --maintenance-policy=MIGRATE \
  --provisioning-model=STANDARD \
  --scopes="${VM_SCOPES}" \
  --image="projects/cos-cloud/global/images/${VM_IMAGE}" \
  --boot-disk-size=10GB \
  --boot-disk-type=pd-balanced \
  --boot-disk-device-name="${INSTANCE_NAME}" \
  --container-image="${IMAGE_NAME}" \
  --container-restart-policy=always \
  --container-env="${CONTAINER_ENVS}" \
  --no-shielded-secure-boot \
  --shielded-vtpm \
  --shielded-integrity-monitoring \
  --labels="${VM_LABEL}"
