#!/bin/bash

set -euo pipefail
set -x

VERSION=$(git describe --tags --abbrev=0 | sed 's/^v//')
echo $VERSION

sed "s/{VERSION}/${VERSION}/g" ./scripts/README_TEMPLATE.md > README.md
