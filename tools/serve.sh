#!/usr/bin/env bash

# 1. Installs `live-server` using npm
# 2. Runs `live-server`
# 3. Runs the tauri dev command

if ! command -v live-server >/dev/null 2>&1; then
    echo "live-server not installed, installing now... (this will run using sudo)..."
    sudo npm install -g live-server
    echo "live-server installed!"
fi

echo "Starting live-server..."
live-server src --port=5555 --no-browser &
live_server_pid=$!

echo "Starting tauri dev..."
cargo tauri dev

# Kill the live-server process when the script exits
trap "kill $live_server_pid" INT
wait
