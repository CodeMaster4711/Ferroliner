#!/bin/sh
set -e

mkdir -p /data

/app/api &
BACKEND_PID=$!

cd /app/frontend
node build &
FRONTEND_PID=$!

trap 'kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit 0' INT TERM

wait $BACKEND_PID $FRONTEND_PID
