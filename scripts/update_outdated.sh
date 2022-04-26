#!/bin/bash

BRANCH_NAME="preview-rs-$1-$2"

if git rev-parse --verify $BRANCH_NAME ; then
    echo "Update branch for $1 -> $2 already exists."
    echo "::set-output name=updated::1"
    exit 0
fi

cargo upgrade "$1@$2"
echo "::set-output name=updated::0"