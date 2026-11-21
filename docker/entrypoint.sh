#!/bin/sh
# Entrypoint for the test containers.
#
# The clipboard tests need a running display server, and docker compose has no way
# to make the main command wait for one (pre-start, post-start, healthcheck are no good).
#
# So this script starts the server chosen by $DISPLAY_SERVER (xvfb or sway),
# waits for it to accept connections, and then delegates to pytest with
# any arguments passed to the container.

set -e

case "$DISPLAY_SERVER" in
  xvfb)
    Xvfb :0 -screen 0 800x600x16 &
    until xdpyinfo >/dev/null 2>&1; do sleep 0.1; done
    ;;
  sway)
    sway --config /dev/null >/dev/null 2>&1 &

    if [ -n "$WAYLAND_DISPLAY" ]; then
      until [ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]; do sleep 0.1; done
    else
      until xdpyinfo >/dev/null 2>&1; do sleep 0.1; done
    fi
    ;;
esac

exec uv run --no-sync pytest "$@"
