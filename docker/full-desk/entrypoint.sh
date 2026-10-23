#!/bin/sh
set -eu

case "${COREVIA_DESK_PORT:-21116}" in
  ''|*[!0-9]*)
    echo "COREVIA_DESK_PORT must be a number between 1 and 65535." >&2
    exit 2
    ;;
esac

if [ "${COREVIA_DESK_PORT}" -lt 1 ] || [ "${COREVIA_DESK_PORT}" -gt 65535 ]; then
  echo "COREVIA_DESK_PORT must be a number between 1 and 65535." >&2
  exit 2
fi

if [ "$#" -eq 0 ]; then
  set -- --server
fi

if [ -z "${DISPLAY:-}" ] && command -v xvfb-run >/dev/null 2>&1; then
  exec xvfb-run --auto-servernum --server-args="-screen 0 1920x1080x24" /opt/corevia/CoreviaDesk "$@"
fi

exec /opt/corevia/CoreviaDesk "$@"
