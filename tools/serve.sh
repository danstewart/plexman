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
live-server src --port=5555 --no-browser >/dev/null &
live_server_pid=$!

# Kill the live-server process on Ctrl-C
trap "kill $live_server_pid" INT

echo "Starting tauri dev..."
cargo tauri dev >/dev/null

# Kill the live-server process when the script exits
echo "Exiting..."
kill $live_server_pid
wait $live_server_pid
