#!/bin/bash

export VM_SCOPES="https://www.googleapis.com/auth/devstorage.read_only,https://www.googleapis.com/auth/logging.write,https://www.googleapis.com/auth/monitoring.write,https://www.googleapis.com/auth/service.management.readonly,https://www.googleapis.com/auth/servicecontrol,https://www.googleapis.com/auth/trace.append"
export VM_IMAGE="cos-stable-117-18613-75-102"
export VM_LABEL="goog-ec-src=vm_add-gcloud,container-vm=${VM_IMAGE}"