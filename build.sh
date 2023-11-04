#!/usr/bin/sh
set -e

if ! type "cargo-skyline" > /dev/null; then
  cargo install --git https://github.com/jam1garner/cargo-skyline
fi

echo "Building release.zip..."

cargo +skyline skyline package -s "https://github.com/RoccoDev/skyline/releases/download/cross-game-local-logging/release.zip"

echo "Building release-version-edit.zip..."

cargo +skyline skyline build --release --features edit-version

cwd=$(pwd)
game_dir="atmosphere/contents/010074F013262000/romfs/skyline/plugins"
target=$cwd/target
cargo_target=${CARGO_TARGET_DIR:-$target}

mkdir -p $target && cd $target
cp release.zip $cargo_target/aarch64-skyline-switch/release/release-version-edit.zip
cd $cargo_target/aarch64-skyline-switch/release
mkdir -p $game_dir
cp libxc3_file_loader.nro $game_dir
zip release-version-edit.zip $game_dir/libxc3_file_loader.nro

cd $target
mv $cargo_target/aarch64-skyline-switch/release/release-version-edit.zip .

cd $cwd

