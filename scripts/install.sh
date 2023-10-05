#!/usr/bin/bash

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
PROJECT_DIR="${SCRIPT_DIR}/.."

BUILD_BIN_FILE="${PROJECT_DIR}/target/debug/remind"
BUILD_BIN_HOTKEY_FILE="${PROJECT_DIR}/target/debug/remind_hc"
BIN_FILE="/usr/local/bin/remind"
BIN_HOTKEY_FILE="/usr/local/bin/remind-hc"
cp $BUILD_BIN_FILE $BIN_FILE
cp $BUILD_BIN_HOTKEY_FILE $BIN_HOTKEY_FILE

exit 0