#!/bin/bash
# Deploys a Cloud Run instance on GCP running the server image (tag: latest)

if [[ ! -n "${POG_REPO:-}" ]];
then
    echo "expects a repository in environment variable: POG_REPO"
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
DISCORD_PUBLIC_KEY_KEY=pog-public_key-$ENVIRONMENT
DB_USER_KEY=pog-db_user-$ENVIRONMENT
DB_PASS_KEY=pog-db_pass-$ENVIRONMENT
DB_HOST_KEY=pog-db_host-$ENVIRONMENT
DB_NAME_KEY=pog-db_name-$ENVIRONMENT

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
if ! DB_USER=$( gcloud secrets versions access latest --secret="$DB_USER_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi
if ! DB_PASS=$( gcloud secrets versions access latest --secret="$DB_PASS_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi
if ! DB_HOST=$( gcloud secrets versions access latest --secret="$DB_HOST_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi
if ! DB_NAME=$( gcloud secrets versions access latest --secret="$DB_NAME_KEY" )
then
    echo "expected secret was not found: $1"
    exit 1
fi

CONTAINER_ENVS="DB_HOST=$DB_HOST,DB_NAME=$DB_NAME,DB_USER=$DB_USER,DB_PASS=$DB_PASS,DISCORD_PUBLIC_KEY=$DISCORD_PUBLIC_KEY,CLIENT_LAMBDA=$CLIENT_LAMBDA,DISCORD_APPLICATION_ID=$DISCORD_APP_ID,APPLICATION_TOKEN=$DISCORD_APP_TOKEN,ENVIRONMENT=$ENVIRONMENT"

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME=pog-server-$ENVIRONMENT-$DEPLOYMENT_TIME
IMAGE_NAME=$POG_REPO/pog_server:latest
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
