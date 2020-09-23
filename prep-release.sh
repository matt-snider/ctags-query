#!/usr/bin/env bash
version=$1
targets=("x86_64-unknown-linux-gnu")

if [[ $# -eq 0 ]] ; then
  echo "Must provide a version (e.g. 0.1.0)"
  exit 1;
fi

cargo_version=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
if [ $cargo_version != $version ] ; then
  echo "Mismatch between provided version ($version) and Cargo.toml version ($cargo_version)"
  exit 1;
fi

# Tag it
echo "Tagging release $version"
git tag "$version"

# Build releases
rm -rf release-tmp/
mkdir release-tmp/

for target in "${targets[*]}"; do
  echo "Building target $target...";
  cargo build --release --target=$target

  cp target/$target/release/ctags-query release-tmp/ctags-query-$version-$target
done

echo "Done. Release assets are in release-tmp/"

