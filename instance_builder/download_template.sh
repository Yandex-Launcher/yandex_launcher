#!/bin/bash

TEMPLATE_DIR="./instance_template"
S3_BUCKET="https://storage.yandexcloud.net/yamcpublic"
ARCHIVE_NAME="minecraft-1_21_8-v2-instance.tar.gz"

if [[ -d $TEMPLATE_DIR ]]; then
    rm -rf $TEMPLATE_DIR
fi

mkdir $TEMPLATE_DIR
curl -L "$S3_BUCKET/$ARCHIVE_NAME" | tar -xz -C $TEMPLATE_DIR
