#!/bin/sh

mkdir -p /storage
chown -R appuser:appuser /storage
exec runuser -u appuser "$@"