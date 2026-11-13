#!/bin/sh
# Builds the student app and the admin dashboard and assembles what the Worker serves (ADR 0006):
#   dist/          the student app (crates/app)
#   dist/admin/    the dashboard (crates/admin), its own bundle, so Learners never download it
# Usage: scripts/build.sh [--dev]   (release by default)
set -eu
cd "$(dirname "$0")/.."
MODE="--release"
[ "${1:-}" = "--dev" ] && MODE=""
(cd crates/app && trunk build $MODE)
(cd crates/admin && trunk build $MODE)
rm -rf dist
mkdir -p dist
cp -R crates/app/dist/. dist/
mkdir -p dist/admin
cp -R crates/admin/dist/. dist/admin/
echo "Built dist/ (app) and dist/admin/ (dashboard)."
