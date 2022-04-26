#!/bin/bash

BRANCH_NAME="preview-rs-$1-$2"

if git rev-parse --verify $BRANCH_NAME ; then
    echo "Update branch for $1 -> $2 already exists."
    exit 0
fi

cargo upgrade "$1@$2"