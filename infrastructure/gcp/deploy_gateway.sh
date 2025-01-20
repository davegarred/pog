#!/bin/bash
# Deploys a VM on GCP running the gateway image (tag: latest)

set -u

SCRIPT_DIR="$(dirname "$(readlink   -f "$0")")"
source "${SCRIPT_DIR}/.env"

if [[ -z "${POG_REPO:-}" ]];
then
    echo "expects a repository in environment variable: POG_REPO"
    exit 1
fi

CONTAINER_ENVS="RUST_BACKTRACE=1,GEMINI_TOKEN=${GEMINI_TOKEN},DISCORD_APPLICATION_ID=${DISCORD_APPLICATION_ID},APPLICATION_TOKEN=${DISCORD_TOKEN},ENVIRONMENT=${ENVIRONMENT}"

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME="pog-gateway-${ENVIRONMENT}-${DEPLOYMENT_TIME}"
IMAGE_NAME="${POG_REPO}/pog_gateway:latest"
VM_SCOPES="https://www.googleapis.com/auth/devstorage.read_only,https://www.googleapis.com/auth/logging.write,https://www.googleapis.com/auth/monitoring.write,https://www.googleapis.com/auth/service.management.readonly,https://www.googleapis.com/auth/servicecontrol,https://www.googleapis.com/auth/trace.append"

gcloud compute instances create-with-container "${INSTANCE_NAME}" \
    --zone=us-central1-c \
    --machine-type=e2-micro \
    --network-interface=network-tier=PREMIUM,stack-type=IPV4_ONLY,subnet=personal-vpc-central1 \
    --maintenance-policy=MIGRATE \
    --provisioning-model=STANDARD \
    --scopes=$VM_SCOPES \
    --image=projects/cos-cloud/global/images/cos-stable-117-18613-75-89 \
    --boot-disk-size=10GB \
    --boot-disk-type=pd-balanced \
    --boot-disk-device-name="${INSTANCE_NAME}" \
    --container-image="${IMAGE_NAME}" \
    --container-restart-policy=always \
    --container-env="$CONTAINER_ENVS" \
    --no-shielded-secure-boot \
    --shielded-vtpm \
    --shielded-integrity-monitoring \
    --labels=goog-ec-src=vm_add-gcloud,container-vm=cos-stable-117-18613-75-89
