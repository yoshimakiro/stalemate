#!/bin/bash

# Get the start time in seconds since the epoch
startTime=$(date +%s)

# Generate a random 256-bit number and hash it
initial=$(openssl rand -hex 32)
lowestHash=$(echo -n $initial | sha256sum | awk '{print $1}')
newHash=lowestHash
# Print the initial value
# echo "Initial hash: $lowestHash"

# Loop 21 times
while true;do
    # Hash the current lowestHash
    #echo $lowestHash | sha256sum | awk '{print $1}'
    newHash=$(echo -n $newHash | sha256sum | awk '{print $1}')

    # Compare hashes numerically
    if [[ "$newHash" < "$lowestHash" ]]; then
        currentTime=$(date +%s)
        elapsedTime=$((currentTime - startTime))
        echo $lowestHash    $elapsedTime 
        lowestHash=$newHash
    fi
done

# Output the final lowest hash
echo "Final lowest hash: $lowestHash"
