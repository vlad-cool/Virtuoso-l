#!/usr/bin/env bash
set -euo pipefail

VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
BIN=Virtuoso
X86="$BIN-$VERSION-x86_64-unknown-linux-gnu"
ARM="$BIN-$VERSION-armv7-unknown-linux-gnueabihf"

confirm() {
    read -r -p "Do you want to continue anyway? [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) echo "Continuing..." ;;
        *) echo "Aborted."; exit 1 ;;
    esac
}

cleanup() { rm -f "$X86" "$ARM"; }
trap cleanup EXIT

if [[ -n "$(git status --porcelain)" ]]; then
    echo "Warning: You have uncommitted Git changes:"
    git status --short
    echo
    confirm
fi

git fetch --quiet
if [[ -n "$(git log '@{u}..HEAD' --oneline 2>/dev/null)" ]]; then
    echo "Warning: You have local commits that are not pushed."
    confirm
fi

if gh release view "v$VERSION" >/dev/null 2>&1; then
    echo "Warning: A release with the tag v$VERSION already exists."
    confirm
fi

echo "Releasing version $VERSION"

cross build --target armv7-unknown-linux-gnueabihf --release \
--no-default-features \
--features=embeded_device,sdl_frontend,legacy_backend_full,gpio_frontend,repeater,cyrano_server
cargo build --release

cp "target/release/$BIN" "$X86"
cp "target/armv7-unknown-linux-gnueabihf/release/$BIN" "$ARM"

gh release create "v$VERSION" \
--target "$(git rev-parse HEAD)" \
--title "Release v$VERSION" \
--notes "Automated release" \
"$X86" "$ARM"