#!/usr/bin/sh

if ! type "cargo-skyline" > /dev/null; then
  cargo install --git https://github.com/jam1garner/cargo-skyline
fi

cargo skyline package -s "https://github.com/RoccoDev/skyline/releases/download/cross-game-local-logging/release.zip"