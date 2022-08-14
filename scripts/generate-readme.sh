#!/bin/bash

set -euo pipefail
set -x

TAG=$(git describe --tags --abbrev=0)
echo $TAG

VERSION=$(echo $TAG | sed 's/^v//')
echo $VERSION

sed "s/{VERSION}/${VERSION}/g" ./scripts/README_TEMPLATE.md > README.md
