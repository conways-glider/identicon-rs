#!/bin/bash

set -euo pipefail

./scripts/generate-readme.sh
git diff --exit-code
