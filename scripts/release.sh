#!/bin/sh

VERSION=$(yq '.package.version' Cargo.toml)

if [[ -n "$(git status --porcelain)" ]]; then
    echo "Warning: You have uncommitted Git changes:"
    git status --short
    echo ""
    read -r -p "Do you want to continue anyway? [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) 
            echo "Continuing..."
            ;;
        *)
            echo "Aborted."
            exit 1
            ;;
    esac
fi

if gh release view "v$VERSION" >/dev/null 2>&1; then
    echo "Warning: Looks like release with such tag already exists:"
    git status --short
    echo ""
    read -r -p "Do you want to continue anyway? [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) 
            echo "Continuing..."
            ;;
        *)
            echo "Aborted."
            exit 1
            ;;
    esac
fi

echo Pushing $VERSION version

cross build --target armv7-unknown-linux-gnueabihf --release --no-default-features --features=embeded_device,sdl_frontend,legacy_backend_full,gpio_frontend,repeater,cyrano_server
cargo build --release

cp target/release/Virtuoso Virtuoso-$VERSION-x86_64-unknown-linux-gnu 
cp target/armv7-unknown-linux-gnueabihf/release/Virtuoso Virtuoso-$VERSION-armv7-unknown-linux-gnueabihf

gh release create v$VERSION \
    --title "Release v$VERSION" \
    --notes "Automated release" \
    Virtuoso-$VERSION-x86_64-unknown-linux-gnu  \
    Virtuoso-$VERSION-armv7-unknown-linux-gnueabihf

rm Virtuoso-$VERSION-x86_64-unknown-linux-gnu
rm Virtuoso-$VERSION-armv7-unknown-linux-gnueabihf
