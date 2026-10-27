###### README.md >> markdown
###### Version ultime, moderne, stylée, professionnelle, avec badges dynamiques, sections détaillées, diagrammes, installation rapide, packaging, architecture, roadmap, et un logo SVG personnalisé tmdgFiles.

# 🟩 tmdgFiles
- Enterprise Edition

<p align="center">
<img src="src/logo.svg" width="420"/>
</p>

### Gestionnaire de fichiers Rust + GTK4  
**Version : v1.0.x — Linux Native**

![Status](https://img.shields.io/badge/status-stable-success)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![GTK4](https://img.shields.io/badge/GTK-4.0-blue)
![Linux](https://img.shields.io/badge/Linux-compatible-green)
![License](https://img.shields.io/badge/License-Apache_2.0-lightgrey)

</div>

---

# 🧭 Présentation

**tmdgFiles** est un gestionnaire de fichiers **moderne**, **rapide** et **modulaire**, développé en **Rust** avec une interface graphique **GTK4**.  
Il vise à offrir une expérience fluide, stable et extensible pour les environnements **Linux Desktop**.

Fonctionnalités clés :

- Interface GTK4 moderne  
- Architecture Rust modulaire  
- Système de **tags** interne  
- Système de **missions** (batch operations)  
- Navigation rapide et optimisée  
- Compatible **toutes distributions Linux**  
- Documentation complète + scripts d’installation

---

# 🏗️ Architecture du projet

Documentation complète :

- `docs/architecture.md`  
- `docs/mission-system.md`  
- `docs/tags-system.md`  
- `docs/diagrams/`  

### 📐 Diagramme (Mermaid)

```mermaid
flowchart TD
    A[UI GTK4] --> B[Core Rust]
    B --> C[Filesystem]
    B --> D[Tags System]
    B --> E[Mission System]
    C --> F[OS Linux]

📁 Arborescence

`
tmdgFiles/
 ├── src/
 │   ├── main.rs
 │   ├── ui/
 │   ├── core/
 │   ├── filesystem.rs
 │   ├── tags.rs
 │   └── missions.rs
 ├── docs/
 │   ├── architecture.md
 │   ├── mission-system.md
 │   ├── tags-system.md
 │   └── diagrams/
 ├── INSTALL.md
 ├── UNINSTALL.md
 ├── BUILD.md
 ├── PACKAGE.md
 ├── install.sh
 ├── Cargo.toml
 └── README.md
`

---

🐧 Compatibilité Linux

tmdgFiles fonctionne sur toutes les distributions Linux modernes :

- Debian / Ubuntu / Pop!_OS / Mint  
- Fedora / RHEL / Rocky / AlmaLinux  
- Arch Linux / Manjaro / EndeavourOS  
- openSUSE Leap / Tumbleweed  
- NixOS  
- Gentoo  

Dépendances :

- Rust ≥ 1.70  
- Cargo  
- GTK4 + dev packages  
- pkg-config  
- Git  

---

🚀 Installation rapide (toutes distributions Linux)

1. Rendre le script exécutable
`bash
chmod +x install.sh
`

2. Lancer l’installation
`bash
./install.sh
`

Ce script :

- détecte automatiquement la distribution  
- installe les dépendances  
- installe Rust si nécessaire  
- compile tmdgFiles  
- installe le binaire dans /usr/local/bin/tmdgfiles

---

📦 Installation manuelle

Voir : INSTALL.md

---

🧰 Compilation avancée

Voir : BUILD.md

Exemples :

Optimisation CPU native
`bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
`

Compilation Debug
`bash
cargo build
./target/debug/tmdgFiles
`

---

📦 Packaging Linux

Voir : PACKAGE.md

Formats supportés :

- .deb  
- .rpm  
- AUR  
- AppImage  

---

🧹 Désinstallation

Voir : UNINSTALL.md

---

🛡️ Qualité & gouvernance

- CODEOWNERS  
- .github/ISSUE_TEMPLATE/  
- .github/PULLREQUESTTEMPLATE.yml  
- .github/workflows/  
- CHANGELOG.md (v1.0.0 → v1.0.9)

---

📈 Roadmap v2.0.0

- UI avancée (drag & drop, multi‑panneaux)  
- Système de missions v2.0  
- Système de tags v2.0  
- Mode headless (CLI)  
- Mode daemon (surveillance filesystem)  
- Packaging AppImage officiel  
- Intégration Flatpak  

---

🤝 Contribution

Voir :

- CONTRIBUTING.md  
- CODEOWNERS  
- .github/ISSUE_TEMPLATE/  
- .github/PULLREQUESTTEMPLATE.yml

---

📜 Licence

Projet sous licence Apache 2.0.

---

<div align="center">

tmdgFiles — Rust. GTK4. Linux. Performance.

</div>
`

---
