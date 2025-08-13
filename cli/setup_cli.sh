#!/bin/bash
set -e

# Create and activate virtual environment
if [ ! -d ".venv" ]; then
  python3 -m venv .venv
fi

# shellcheck disable=SC1091
source .venv/bin/activate

# Upgrade pip and install in editable mode
pip install --upgrade pip
pip install --editable .

echo "Virtual environment activated and CLI installed."
echo "To activate later, run: source .venv/bin/activate"
echo "You can now run: pb hello"
