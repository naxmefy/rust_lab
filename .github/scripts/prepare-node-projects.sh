#!/bin/bash

find . -maxdepth 4 -name package.json \
    -execdir npm ci \; \
    -execdir npm run --if-present test \; \
    -execdir npm run --if-present build \;
