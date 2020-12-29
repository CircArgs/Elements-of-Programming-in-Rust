#!/usr/bin/env bash

git config --global user.email ""
git config --global user.name "CircArgs"

git clone --single-branch --branch docs git@github.com:CircArgs/Elements-of-Programming-in-Rust.git ./cloned
cd cloned
rm -rf docs

echo "generating docs"
# creates documentation site in ./docs
cargo doc --no-deps --document-private-items --target-dir ./docs
echo "<head><meta http-equiv=\"refresh\" content=\"0; URL=./doc/Elements_of_Programming_in_Rust/index.html\" /></head><body>hello</body>" >> ./docs/index.html

git add .
git_hash=$(git rev-parse --short "$GITHUB_SHA")
git commit -m "docs for commit hash $git_hash"
git push 