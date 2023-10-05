#!/usr/bin/bash
echo "building remind hotkey client [src/remind_hotkey_client.rs] ..."
rustc -o target/debug/remind_hc src/remind_hotkey_client.rs 
echo "remind hotkey client binary [target/debug/remind-hc] built successfuly!"

echo "building remind app [src/main.rs] ..."
cargo build
echo "remind app [target/debug/remind] built successfuly!"

exit 0