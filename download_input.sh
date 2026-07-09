#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

YEAR=$1
DAY=$2
DAY_PADDED=$(printf "%02d" ${DAY})

URL="https://flipflop.slome.org/${YEAR}/${DAY}/input"
OUTPUT_DIR="/home/pooh/flipflop/input"
OUTPUT_FILE="${OUTPUT_DIR}/flipflop_codes_${YEAR}_${DAY_PADDED}.txt"

mkdir -p "${OUTPUT_DIR}"

echo "Downloading input for Year: ${YEAR}, Day: ${DAY}..."
curl -s --cookie `cat ${HOME}/flipflop/.cookie` "${URL}" -o "${OUTPUT_FILE}"

if [ $? -eq 0 ]; then
    echo "Input downloaded successfully to ${OUTPUT_FILE}"
else
    echo "Error downloading input."
fi
