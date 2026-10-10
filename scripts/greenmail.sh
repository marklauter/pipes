#!/bin/sh
# Starts or stops a GreenMail test mail server in Podman.
# SMTP 3025, POP3 3110, IMAP 3143, SMTPS 3465, IMAPS 3993, POP3S 3995, API 8080.
# Auth is disabled: any login works, and a mailbox is created on first delivery.
set -eu

name=pipes-greenmail
image=docker.io/greenmail/standalone:2.1.3

case "${1:-up}" in
  up)
    podman rm -f "$name" >/dev/null 2>&1 || true
    podman run -d --name "$name" \
      -p 3025:3025 -p 3110:3110 -p 3143:3143 \
      -p 3465:3465 -p 3993:3993 -p 3995:3995 -p 8080:8080 \
      "$image" >/dev/null
    echo "started $name"
    ;;
  down)
    podman rm -f "$name" >/dev/null
    echo "stopped $name"
    ;;
  logs)
    podman logs "$name"
    ;;
  *)
    echo "usage: $0 [up|down|logs]" >&2
    exit 2
    ;;
esac
