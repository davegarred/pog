#!/bin/bash
# Deploys a Cloud Run instance on GCP running the client image (tag: latest)

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

DEPLOYMENT_TIME=$( date +%y%m%d%H%M )
INSTANCE_NAME=pog-client-$ENVIRONMENT-$DEPLOYMENT_TIME
IMAGE_NAME=$POG_REPO/pog_client:latest

gcloud run deploy $INSTANCE_NAME \
--image=$IMAGE_NAME \
--allow-unauthenticated \
--port=8080 \
--execution-environment=gen1 \
--ingress=internal \
--region=us-central1
