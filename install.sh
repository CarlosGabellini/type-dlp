#!/usr/bin/env bash
# type-dlp installer (Linux x86_64)
# Installs: ffmpeg, yt-dlp and type-dlp into /usr/local/bin

set -euo pipefail

REPO="CarlosGabellini/type-dlp"
BIN_NAME="type-dlp"
INSTALL_DIR="/usr/local/bin"

TYPE_DLP_URL="https://github.com/CarlosGabellini/type-dlp/releases/download/v1.0.0/type-dlp"
YTDLP_URL="https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux"

# ---------- helpers ----------

info()  { echo -e "\033[1;34m==>\033[0m $1"; }
error() { echo -e "\033[1;31merror:\033[0m $1" >&2; exit 1; }

have() { command -v "$1" >/dev/null 2>&1; }

# ---------- uninstall ----------

if [ "${1:-}" = "--uninstall" ]; then
    info "Removing type-dlp and yt-dlp from ${INSTALL_DIR} (ffmpeg is left untouched)"
    sudo rm -f "${INSTALL_DIR}/type-dlp" "${INSTALL_DIR}/yt-dlp"
    info "Done."
    exit 0
fi

# ---------- checks ----------

[ "$(uname -s)" = "Linux" ]   || error "This installer only supports Linux."
[ "$(uname -m)" = "x86_64" ]  || error "This installer only supports x86_64."
have curl                     || error "curl is required. Install it and run this script again."
have sudo                     || error "sudo is required."

info "This script will use sudo to install ffmpeg, yt-dlp and type-dlp."
read -r -p "Continue? [y/N] " answer < /dev/tty
case "$answer" in
    y|Y|yes|YES) ;;
    *) echo "Cancelled."; exit 0 ;;
esac

# ---------- ffmpeg ----------

if have ffmpeg; then
    info "ffmpeg already installed, skipping."
else
    info "Installing ffmpeg..."
    if have apt-get; then
        sudo apt-get update
        sudo apt-get install -y ffmpeg
    elif have pacman; then
        sudo pacman -S --needed --noconfirm ffmpeg
    elif have dnf; then
        sudo dnf install -y ffmpeg
    else
        error "No supported package manager found (apt-get, pacman, dnf). Install ffmpeg manually."
    fi
fi

# ---------- yt-dlp ----------

if have yt-dlp; then
    info "yt-dlp already installed, skipping. (Update it with: sudo yt-dlp -U)"
else
    info "Installing yt-dlp..."
    sudo curl -fL "$YTDLP_URL" -o "${INSTALL_DIR}/yt-dlp"
    sudo chmod +x "${INSTALL_DIR}/yt-dlp"
fi

# ---------- type-dlp ----------

info "Installing type-dlp..."
sudo curl -fL "$TYPE_DLP_URL" -o "${INSTALL_DIR}/type-dlp"
sudo chmod +x "${INSTALL_DIR}/type-dlp"

info "All set! Run it with: type-dlp"