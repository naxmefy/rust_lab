#!/bin/bash

find . -maxdepth 4 -name package.json -execdir npm ci \;
find . -maxdepth 4 -name package.json -execdir npm run --if-present test \;
find . -maxdepth 4 -name package.json -execdir npm run --if-present build \;
