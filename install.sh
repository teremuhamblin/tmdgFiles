#!/usr/bin/env bash

set -e

echo "[tmdgFiles] Détection de la distribution..."

# --- Détection distribution ---
if [ -f /etc/debian_version ]; then
    DISTRO="debian"
elif [ -f /etc/fedora-release ]; then
    DISTRO="fedora"
elif [ -f /etc/arch-release ]; then
    DISTRO="arch"
elif [ -f /etc/SuSE-release ] || [ -f /etc/os-release ] && grep -qi "suse" /etc/os-release; then
    DISTRO="opensuse"
else
    echo "Distribution non reconnue."
    exit 1
fi

echo "[tmdgFiles] Distribution détectée : $DISTRO"

# --- Installation dépendances ---
case "$DISTRO" in
    debian)
        sudo apt update
        sudo apt install -y git curl build-essential libgtk-4-dev
        ;;
    fedora)
        sudo dnf install -y git curl gcc gcc-c++ make gtk4-devel
        ;;
    arch)
        sudo pacman -Syu --noconfirm
        sudo pacman -S --noconfirm git curl base-devel gtk4
        ;;
    opensuse)
        sudo zypper install -y git curl gcc-c++ make gtk4-devel
        ;;
esac

# --- Installation Rust ---
if ! command -v cargo >/dev/null 2>&1; then
    echo "[tmdgFiles] Installation de Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# --- Clonage du projet ---
echo "[tmdgFiles] Clonage du dépôt..."
git clone https://github.com/teremuhamblin/tmdgFiles
cd tmdgFiles

# --- Compilation ---
echo "[tmdgFiles] Compilation..."
cargo build --release

# --- Installation globale ---
echo "[tmdgFiles] Installation dans /usr/local/bin..."
sudo cp target/release/tmdgFiles /usr/local/bin/tmdgfiles

echo "[tmdgFiles] Installation terminée."
echo "Lancez : tmdgfiles"
