#!/usr/bin/env bash
# Re-applies the mechanical Corevia Desk branding (app ids, names, platform identity, icons).
# Idempotent: running it twice changes nothing. See .corevia/branding/apply.py.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
exec python3 .corevia/branding/apply.py
