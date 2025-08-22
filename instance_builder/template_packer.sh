#!/bin/bash

if [[ $# -eq 0 ]]; then
    echo "No minecraft directory specified"
    exit 1
fi
MINECRAFT_DIR_PATH="$1"
if [[ ! -d "$MINECRAFT_DIR_PATH" ]]; then
    echo "\"$MINECRAFT_DIR_PATH\" doesn't exist or not a directory"
    exit 1
fi

ARCHIVE_OUT="minecraft.tar.gz"

jq -r '.versions[0].include | map(.path) | .[]' "spec.json" \
| tar -C "$MINECRAFT_DIR_PATH" \
      --transform 's,^,minecraft/,' \
      -czf "$ARCHIVE_OUT" \
      -T -
