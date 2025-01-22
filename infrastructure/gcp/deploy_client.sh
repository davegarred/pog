#!/bin/bash
# Deploys a Cloud Run instance on GCP running the client image (tag: latest)

set -u

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
source "${SCRIPT_DIR}/.env"
source "${SCRIPT_DIR}/common.sh"

if [[ -z "${POG_REPO:-}" ]];
then
    echo "expects a repository in environment variable: POG_REPO"
    exit 1
fi

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME="pog-client-${ENVIRONMENT}-${DEPLOYMENT_TIME}"
IMAGE_NAME="${POG_REPO}/pog_client:latest"

CONTAINER_ENVS="RUST_BACKTRACE=1,ENVIRONMENT=${ENVIRONMENT}"

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
  --tags=http-server \
  --labels="${VM_LABEL}"
