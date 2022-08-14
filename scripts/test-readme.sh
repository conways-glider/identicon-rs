#!/bin/bash

set -euo pipefail
set -x

./scripts/generate-readme.sh
git diff --exit-code
