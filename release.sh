#!/bin/bash

ARG_VERSION=$1
ARG_PLATFORMS=$2

if [[ -z "$ARG_VERSION" || -z "$ARG_PLATFORMS" ]]; then
    echo "Usage: $0<version> <flags>"
    echo "Example: $0 1-dev-console lmw"
    echo "Flags: l=Linux, m=macOS, w=Windows"
    exit 1
fi

list_file() {
    local path=$1
    ls -lh "$path"; ls -lh --si "$path"; ls -l "$path"
}

for (( i=0; i<${#ARG_PLATFORMS}; i++ )); do
    char="${ARG_PLATFORMS:$i:1}"

    case $char in
        l)
            echo
            echo "--- Building Linux (version: $ARG_VERSION) ---"
            TARGETS=("x86_64-unknown-linux-gnu" "x86_64-unknown-linux-musl")
            for TARGET in "${TARGETS[@]}"; do
                cargo clean && cargo build --config .cargo/config.nightly.toml --release --target "$TARGET"
                strip "target/$TARGET/release/color-columns-tui"
                list_file "target/$TARGET/release/color-columns-tui"
                cp -uv "target/$TARGET/release/color-columns-tui" "binaries/Linux/color-columns-tui__${TARGET}_${ARG_VERSION}"
                echo
            done
            ;;

        m)
            echo
            echo "--- Building macOS (version: $ARG_VERSION) ---"
            mkdir -p /tmp/dummy_sdk/usr/lib
            TARGETS=("x86_64-apple-darwin" "aarch64-apple-darwin")
            for TARGET in "${TARGETS[@]}"; do
                cargo clean
                SDKROOT=/tmp/dummy_sdk cargo zigbuild --config .cargo/config.nightly.toml --release --target "$TARGET"
                list_file "target/$TARGET/release/color-columns-tui"
                cp -uv "target/$TARGET/release/color-columns-tui" "binaries/macOS/color-columns-tui__${TARGET}_${ARG_VERSION}"
                echo
            done

            llvm-lipo-18 -create \
                "binaries/macOS/color-columns-tui__x86_64-apple-darwin_${ARG_VERSION}" \
                "binaries/macOS/color-columns-tui__aarch64-apple-darwin_${ARG_VERSION}" \
                -output "binaries/macOS/color-columns-tui__macOS-universal_${ARG_VERSION}"
            list_file "binaries/macOS/color-columns-tui__macOS-universal_${ARG_VERSION}"
            rm -f "binaries/macOS/color-columns-tui__x86_64-apple-darwin_${ARG_VERSION}" "binaries/macOS/color-columns-tui__aarch64-apple-darwin_${ARG_VERSION}"
            echo
            ;;

        w)
            echo
            echo "--- Building Windows (version: $ARG_VERSION) ---"
            TARGET="x86_64-pc-windows-gnu"
            cargo clean && cargo build --config .cargo/config.nightly.toml --release --target "$TARGET"
            x86_64-w64-mingw32-strip "target/$TARGET/release/color-columns-tui.exe"
            list_file "target/$TARGET/release/color-columns-tui.exe"
            cp -uv "target/$TARGET/release/color-columns-tui.exe" "binaries/Windows/color-columns-tui.exe__${TARGET}_${ARG_VERSION}"
            echo
            ;;

        *)
            echo "Warning: Unknown platform argument '$char' ignored."
            ;;
    esac
done
