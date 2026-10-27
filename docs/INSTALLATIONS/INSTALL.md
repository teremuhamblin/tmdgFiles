###### INSTALL.md >> markdown
# 📦 tmdgFiles
- Gestionnaire de fichiers Rust + GTK4
- Version : v1.0.0

### 🧭 Introduction
**tmdgFiles** est un gestionnaire de fichiers moderne, rapide et modulaire, développé en Rust avec une interface GTK4.  
Ce guide explique toutes les méthodes d’installation, pour toutes les distributions Linux, du plus simple au plus avancé.

---
---

### 🛠️ 1. Prérequis système
- tmdgFiles nécessite :
```text
- Rust ≥ 1.70  
- Cargo (inclus avec Rust)  
- GTK4 + dépendances de développement  
- Git  
- Un environnement Linux compatible (toutes les distributions modernes)
```

### 🐧 2. Installation des dépendances selon la distribution

- ✔️ **Debian / Ubuntu / Linux Mint / Pop!_OS**
```text
sudo apt update
sudo apt install -y git curl build-essential libgtk-4-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

- ✔️ **Fedora / RHEL / Rocky / AlmaLinux**
```text
sudo dnf install -y git curl gcc gcc-c++ make gtk4-devel
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

- ✔️ **Arch Linux / Manjaro / EndeavourOS**
```text
sudo pacman -Syu --noconfirm
sudo pacman -S --noconfirm git curl base-devel gtk4
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

- ✔️ **openSUSE Leap / Tumbleweed**
```text
sudo zypper install -y git curl gcc-c++ make gtk4-devel
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

- ✔️ **NixOS / LegionOS**
```text
nix-shell -p rustc cargo gtk4 git
```

### 📥 3. Récupération du projet
```text
git clone https://github.com/teremuhamblin/tmdgFiles
cd tmdgFiles
```

### ⚙️ 4. Compilation

- **Compilation standard** (*toutes distributions*)
```text
cargo build --release
```

>> Le binaire final se trouve dans :  
```text
target/release/tmdgFiles
```

### 🚀 5. Exécution
```text
./target/release/tmdgFiles
```

### 🧩 6. Installation globale (*optionnelle*)

- >> **Méthode 1 — Installation dans /usr/local/bin**
```text
sudo cp target/release/tmdgFiles /usr/local/bin/tmdgfiles
```

- *Puis exécution* :  
```text
tmdgfiles
```

- >> **Méthode 2 — Installation via Cargo** (*développement*)
```text
cargo install --path .
```

### 🎨 7. Options d’installation avancées

🔹 **Mode Debug**
```md
cargo build
./target/debug/tmdgFiles
```

🔹 **Nettoyage complet**
```md
cargo clean
```

🔹 **Compilation optimisée CPU** (*avancé*)
```md
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

🔹 **Compilation sans GTK4** (*mode headless futur*)
   - ***Réservé aux versions futures, non disponible en v1.0.0***

### 🧪 8. Vérification de l’environnement
```text
rustc --version
cargo --version
pkg-config --modversion gtk4
```

### 🐛 9. Dépannage

> GTK4 introuvable
- *Installer les paquets* **libgtk-4-dev, gtk4-devel ou gtk4**.

> Erreur pkg-config
```text
sudo apt install pkg-config
```

> Rust non détecté
```text
source $HOME/.cargo/env
```

### 📌 10. Désinstallation

- *Si installé via /usr/local/bin*
```text
sudo rm /usr/local/bin/tmdgfiles
```

- *Si installé via Cargo*
```text
cargo uninstall tmdgfiles
```

---

### 🏁 Conclusion
```md
Ce guide couvre toutes les méthodes d’installation, pour toutes les distributions Linux, du mode simple au mode avancé.  
tmdgFiles est compatible avec 100% des distributions Linux modernes supportant GTK4 et Rust.
```

---
---
