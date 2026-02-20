#!/bin/bash

# If any command fails, stop the script immediately 
set -e

# Required packages to compile the project 
REQUIRED_PACKAGES=(
  pkg-config
  libssl-dev
  build-essential
)

echo "This script requires root privileges to install the following system packages:"
echo
for pkg in "${REQUIRED_PACKAGES[@]}"; do
  echo " • $pkg"
done
echo

sudo apt update 
sudo apt install pkg-config libssl-dev build-essential
make
