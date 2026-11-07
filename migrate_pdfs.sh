#!/bin/bash
set -e

SOURCE_DIRS=("/c/Users/casey/polln/constrainttheory/docs/reports/" "/c/Users/casey/polln/constrainttheory/research/")
TARGET_DIR="/c/Users/casey/polln/SuperInstance-papers/research/"

mkdir -p "$TARGET_DIR"

BYTES_RECLAIMED=0

# Find all PDFs in the source directories
# Use null terminator to handle spaces perfectly
find "${SOURCE_DIRS[@]}" -name "*.pdf" -print0 | while IFS= read -r -d '' FILE; do
    if [[ "$FILE" == *" (1).pdf" ]]; then
        # It's a duplicate
        SIZE=$(stat -c%s "$FILE")
        rm "$FILE"
        echo "[DELETING] $FILE"
        # Since we are in a subshell from the while loop, we can't modify BYTES_RECLAIMED outside
        # unless we use a temporary file.
        echo "$SIZE" >> /tmp/bytes_reclaimed.tmp
    else
        # It's a primary file, move it
        FILENAME=$(basename "$FILE")
        echo "[MOVING] $FILE -> $TARGET_DIR$FILENAME"
        mv "$FILE" "$TARGET_DIR$FILENAME"
    fi
done

# Calculate total bytes reclaimed
if [ -f /tmp/bytes_reclaimed.tmp ]; then
    BYTES_RECLAIMED=$(awk '{s+=$1} END {print s}' /tmp/bytes_reclaimed.tmp)
    rm /tmp/bytes_reclaimed.tmp
else
    BYTES_RECLAIMED=0
fi

echo "MIGRATION_COMPLETE"
echo "BYTES_RECLAIMED=$BYTES_RECLAIMED"
