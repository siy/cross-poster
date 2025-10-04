#!/bin/sh
# Installation script for article-cross-poster
# Usage: curl -fsSL https://raw.githubusercontent.com/<owner>/article-cross-poster/main/install.sh | sh

set -e

# Configuration
REPO="cross-poster"
BINARY_NAME="article-cross-poster"
GITHUB_USER="${GITHUB_USER:-siy}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
VERSION="${VERSION:-latest}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
    exit 1
}

# Detect OS and architecture
detect_platform() {
    local os arch platform

    # Detect OS
    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        MINGW*|MSYS*|CYGWIN*)
            error "Windows is not supported by this installer. Please download from GitHub releases."
            ;;
        *)
            error "Unsupported operating system: $(uname -s)"
            ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            error "Unsupported architecture: $(uname -m)"
            ;;
    esac

    platform="${BINARY_NAME}-${os}-${arch}"
    echo "$platform"
}

# Get the latest release version
get_latest_version() {
    local latest_url="https://api.github.com/repos/${GITHUB_USER}/${REPO}/releases/latest"

    if command -v curl >/dev/null 2>&1; then
        curl -s "$latest_url" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$latest_url" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/'
    else
        error "curl or wget is required"
    fi
}

# Download and install
main() {
    info "Article Cross-Poster Installer"
    info "=============================="

    # Check for required commands
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        error "curl or wget is required but not installed"
    fi

    if ! command -v tar >/dev/null 2>&1; then
        error "tar is required but not installed"
    fi

    # Detect platform
    local platform
    platform=$(detect_platform)
    info "Detected platform: $platform"

    # Determine version
    local version
    if [ "$VERSION" = "latest" ]; then
        info "Fetching latest version..."
        version=$(get_latest_version)
        if [ -z "$version" ]; then
            error "Failed to fetch latest version"
        fi
    else
        version="$VERSION"
    fi
    info "Installing version: v$version"

    # Construct download URL
    local archive_name="${platform}.tar.gz"
    local download_url="https://github.com/${GITHUB_USER}/${REPO}/releases/download/v${version}/${archive_name}"
    local checksum_url="${download_url}.sha256"

    # Create temporary directory
    local tmp_dir
    tmp_dir=$(mktemp -d 2>/dev/null || mktemp -d -t 'article-cross-poster')
    trap 'rm -rf "$tmp_dir"' EXIT

    info "Downloading from: $download_url"

    # Download archive
    local archive_path="${tmp_dir}/${archive_name}"
    if command -v curl >/dev/null 2>&1; then
        if ! curl -L -f -o "$archive_path" "$download_url"; then
            error "Download failed. Please check the version and GitHub username."
        fi
    else
        if ! wget -O "$archive_path" "$download_url"; then
            error "Download failed. Please check the version and GitHub username."
        fi
    fi

    # Download and verify checksum
    info "Verifying checksum..."
    local checksum_path="${tmp_dir}/checksum.sha256"
    if command -v curl >/dev/null 2>&1; then
        curl -L -f -o "$checksum_path" "$checksum_url" 2>/dev/null || warn "Checksum file not found, skipping verification"
    else
        wget -O "$checksum_path" "$checksum_url" 2>/dev/null || warn "Checksum file not found, skipping verification"
    fi

    if [ -f "$checksum_path" ]; then
        cd "$tmp_dir"
        if command -v shasum >/dev/null 2>&1; then
            if ! shasum -a 256 -c "$checksum_path" >/dev/null 2>&1; then
                error "Checksum verification failed"
            fi
        elif command -v sha256sum >/dev/null 2>&1; then
            if ! sha256sum -c "$checksum_path" >/dev/null 2>&1; then
                error "Checksum verification failed"
            fi
        else
            warn "sha256sum/shasum not available, skipping checksum verification"
        fi
        cd - >/dev/null
        info "Checksum verified successfully"
    fi

    # Extract archive
    info "Extracting archive..."
    tar -xzf "$archive_path" -C "$tmp_dir"

    # Determine installation directory
    local final_install_dir="$INSTALL_DIR"

    # Check if we can write to the install directory
    if [ ! -w "$final_install_dir" ] && [ "$final_install_dir" = "/usr/local/bin" ]; then
        warn "Cannot write to $final_install_dir, trying \$HOME/.local/bin instead"
        final_install_dir="$HOME/.local/bin"
        mkdir -p "$final_install_dir"
    fi

    # Install binary
    local binary_path="${tmp_dir}/${BINARY_NAME}"
    local install_path="${final_install_dir}/${BINARY_NAME}"

    info "Installing to: $install_path"

    if [ -w "$final_install_dir" ]; then
        cp "$binary_path" "$install_path"
        chmod +x "$install_path"
    else
        # Need sudo
        info "Requesting sudo access for installation..."
        sudo cp "$binary_path" "$install_path"
        sudo chmod +x "$install_path"
    fi

    # Verify installation
    if [ -x "$install_path" ]; then
        info "Installation successful!"
        info ""
        info "Run '${BINARY_NAME} --help' to get started"
        info "Run '${BINARY_NAME} config init' to set up your API credentials"

        # Check if install dir is in PATH
        case ":$PATH:" in
            *":$final_install_dir:"*) ;;
            *)
                warn ""
                warn "$final_install_dir is not in your PATH"
                warn "Add it to your PATH by adding this to your shell profile:"
                warn "  export PATH=\"\$PATH:$final_install_dir\""
                ;;
        esac
    else
        error "Installation verification failed"
    fi
}

main "$@"
