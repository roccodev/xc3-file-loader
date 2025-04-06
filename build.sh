#!/usr/bin/bash
set -e

if ! type "cargo-skyline" > /dev/null; then
  cargo install --git https://github.com/roccodev/cargo-skyline --branch minor/remove-npdm-extraction
  cargo-skyline skyline update-std
fi

skyline_url="https://github.com/roccodev/skyline/releases/download/mod0-align/release.zip"
cwd=$(pwd)
target=$cwd/target
cargo_target=${CARGO_TARGET_DIR:-$target}
out_base="xcnx-file-loader"
npdmtool=${NPDMTOOL:-$(which npdmtool)}

results=(
    "xc2-ww 0100E95004038000 menu/font/standard.wifnt"
    "xc2-jp 0100F3400332C000 menu/font/standard.wifnt"
    "torna 0100C9F009F7A000 menu_ira/font/standard.wifnt"
    "xc3 010074F013262000 menu/font/standard.wifnt"
    "xcxde 0100453019AA8000 ui/font/unique.wifnt"
)

mkdir -p "$cargo_target/skyline-pkg"
mkdir -p "$cargo_target/skyline-pkg/npdm"
mkdir -p "$cargo_target/skyline-pkg/tester-zip"

for x in "${results[@]}"
do
  set -- $x
  zip_path="$cargo_target/skyline-pkg/$out_base-$1.zip"
  game_dir="atmosphere/contents/$2"
  out_npdm="$cargo_target/skyline-pkg/npdm/$1.npdm"

  echo "Building $out_base-$1.zip..."
  cargo skyline package --skyline-release "$skyline_url" -t $2 -o "$zip_path"
  echo "Building $1.npdm..."
  "$NPDMTOOL" "res/npdm/$1.json" "$out_npdm"
  zip "$zip_path" "$out_npdm"
  printf "@ ${out_npdm#/}\n@=$game_dir/exefs/main.npdm\n" | zipnote -w "$zip_path"

  # Tester zip
  unzip -o -d "$cargo_target/tester-zip" "$zip_path"
  tester_base="$cargo_target/tester-zip/$game_dir/romfs"
  mkdir -p $(dirname "$tester_base/$3")
  cp res/test/test_font.wifnt "$tester_base/$3"
done

echo "Building tester zip..."
cd "$cargo_target/tester-zip" 
zip -r test-all.zip .
mv test-all.zip ../skyline-pkg
cd -

cd $cwd

