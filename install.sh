#!/usr/bin/env bash
set -euo pipefail

REPO="maxischmaxi/ping"
BINARY="pong"
INSTALL_DIR="/usr/local/bin"

echo "Installing $BINARY..."

# Detect OS
OS="$(uname -s)"
case "$OS" in
  Linux)  OS_TAG="unknown-linux-gnu" ;;
  Darwin) OS_TAG="apple-darwin" ;;
  *)
    echo "Error: Unsupported OS: $OS" >&2
    exit 1
    ;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64)         ARCH_TAG="x86_64" ;;
  aarch64|arm64)  ARCH_TAG="aarch64" ;;
  *)
    echo "Error: Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

TARGET="${ARCH_TAG}-${OS_TAG}"

# Get latest release tag
LATEST="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | cut -d '"' -f 4)"

if [ -z "$LATEST" ]; then
  echo "Error: Could not determine latest release." >&2
  exit 1
fi

echo "Latest release: $LATEST"

ARCHIVE="${BINARY}-${LATEST}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${LATEST}/${ARCHIVE}"

# Download and install
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading $URL..."
curl -fsSL "$URL" -o "${TMPDIR}/${ARCHIVE}"

tar xzf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR"

install -m 755 "${TMPDIR}/${BINARY}" "${INSTALL_DIR}/${BINARY}"

# Set raw socket capability on Linux so pong can run without sudo
if [ "$OS" = "Linux" ] && command -v setcap &> /dev/null; then
  setcap cap_net_raw+ep "${INSTALL_DIR}/${BINARY}" 2>/dev/null || true
fi

echo "$BINARY $LATEST installed to ${INSTALL_DIR}/${BINARY}"
