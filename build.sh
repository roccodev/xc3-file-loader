#!/usr/bin/bash
set -e

if ! type "cargo-skyline" > /dev/null; then
  cargo install --git https://github.com/jam1garner/cargo-skyline
  cargo-skyline skyline update-std 
fi

skyline_url="https://github.com/roccodev/skyline/releases/download/cross-game-local-logging/release.zip"
cwd=$(pwd)
target=$cwd/target
cargo_target=${CARGO_TARGET_DIR:-$target}
out_base="xcnx-file-loader"
npdmtool=${NPDMTOOL:-$(which npdmtool)}

results=(
    "xc2-ww 0100E95004038000"
    "xc2-jp 0100F3400332C000"
    "torna 0100C9F009F7A000"
    "xc3 010074F013262000"
    "xcxde 0100453019AA8000"
)

mkdir -p "$cargo_target/skyline-pkg"
mkdir -p "$cargo_target/skyline-pkg/npdm"

for x in "${results[@]}"
do
  set -- $x
  zip_path="$cargo_target/skyline-pkg/$out_base-$1.zip"
  game_dir="atmosphere/contents/$2"
  out_npdm="$cargo_target/skyline-pkg/npdm/$1.npdm"

  echo "Building $out_base-$1.zip..."
  cargo skyline package -t $2 -o "$zip_path"
  echo "Building $1.npdm..."
  "$NPDMTOOL" "res/npdm/$1.json" "$out_npdm"
  zip "$zip_path" "$out_npdm"
  printf "@ ${out_npdm#/}\n@=$game_dir/exefs/main.npdm\n" | zipnote -w "$zip_path"
done

cd $cwd

