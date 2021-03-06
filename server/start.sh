#!/bin/sh

touch /app/log/server.log
exec /app/server "${@}"
