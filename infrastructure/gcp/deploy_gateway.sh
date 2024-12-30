#!/bin/bash
# Deploys a VM on GCP running the gateway image (tag: latest)

if [[ ! -n "${POG_REPO:-}" ]];
then
    echo "expects a repository in env: POG_REPO"
    exit 1
fi
if [[ $# -ne 1 ]];
then
    echo "usage:"
    echo "  $0 <environment>"
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

CONTAINER_ENVS="GEMINI_TOKEN=$GEMINI_TOKEN,CLIENT_LAMBDA=$CLIENT_LAMBDA,DISCORD_APPLICATION_ID=$DISCORD_APP_ID,APPLICATION_TOKEN=$DISCORD_APP_TOKEN,ENVIRONMENT=$ENVIRONMENT"

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME=gateway-$ENVIRONMENT-$DEPLOYMENT_TIME
IMAGE_NAME=$POG_REPO/gateway:latest
VM_SCOPES="https://www.googleapis.com/auth/devstorage.read_only,https://www.googleapis.com/auth/logging.write,https://www.googleapis.com/auth/monitoring.write,https://www.googleapis.com/auth/service.management.readonly,https://www.googleapis.com/auth/servicecontrol,https://www.googleapis.com/auth/trace.append"

gcloud compute instances create-with-container $INSTANCE_NAME \
    --zone=us-central1-c \
    --machine-type=e2-micro \
    --network-interface=network-tier=PREMIUM,stack-type=IPV4_ONLY,subnet=personal-vpc-central1 \
    --maintenance-policy=MIGRATE \
    --provisioning-model=STANDARD \
    --scopes=$VM_SCOPES \
    --image=projects/cos-cloud/global/images/cos-stable-117-18613-75-72 \
    --boot-disk-size=10GB \
    --boot-disk-type=pd-balanced \
    --boot-disk-device-name=$INSTANCE_NAME \
    --container-image=$IMAGE_NAME \
    --container-restart-policy=always \
    --container-env="$CONTAINER_ENVS" \
    --no-shielded-secure-boot \
    --shielded-vtpm \
    --shielded-integrity-monitoring \
    --labels=goog-ec-src=vm_add-gcloud,container-vm=cos-stable-117-18613-75-72
