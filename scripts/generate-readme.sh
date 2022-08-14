#!/bin/bash

set -euo pipefail

VERSION=$(git describe --tags | sed 's/^v//')
echo $VERSION

sed "s/{VERSION}/${VERSION}/g" ./scripts/README_TEMPLATE.md > README.md
