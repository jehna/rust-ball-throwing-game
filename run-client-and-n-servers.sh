#!/bin/bash

NUM_CLIENTS="${1:-1}"

# Kill all processes when this script exits
trap "kill 0" EXIT

echo "Running server..."
./run-server.sh &

echo "Running $NUM_CLIENTS clients..."
for ((i=0; i<NUM_CLIENTS; i++)); do
    echo "Running client $i..."
    ./run-client.sh &
done

echo "Done! All systems running."

# Wait for all processes to finish
wait