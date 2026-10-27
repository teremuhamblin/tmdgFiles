###### PACKAGE.md >> markdown
# 🟥 tmdgFiles
- — Packaging Linux (*création .deb, .rpm, AUR*)

### 📦 1. Création d’un paquet **.deb** (Debian / Ubuntu)

- >> Installer cargo-deb
```bash
cargo install cargo-deb
```

- >> Générer le .deb
```bash
cargo deb
```

>> Le fichier sera dans :
```md
target/debian/*.deb
```

---

### 📦 2. Création d’un paquet **.rpm** (*Fedora / RHEL / openSUSE*)

- >> Installer cargo-rpm
```bash
cargo install cargo-rpm
```

- >> Initialiser
```bash
cargo rpm init
```

- >> Construire
```bash
cargo rpm build
```

>> Le fichier sera dans :
```md
target/release/rpmbuild/RPMS/x86_64/*.rpm
```

---

### 📦 3. Création d’un paquet **AUR** (*Arch Linux*)
- Structure AUR
- >> Créer un dossier :
```text
tmdgfiles/
 ├── PKGBUILD
 ├── .SRCINFO
 └── tmdgfiles.install (optionnel)
```

- >> Exemple de PKGBUILD minimal :
```bash
pkgname=tmdgfiles
pkgver=1.0.0
pkgrel=1
arch=('x86_64')
depends=('gtk4')
makedepends=('cargo')
source=("git+https://github.com/teremuhamblin/tmdgFiles")
build() {
    cd "$srcdir/tmdgFiles"
    cargo build --release
}
package() {
    install -Dm755 "$srcdir/tmdgFiles/target/release/tmdgFiles" "$pkgdir/usr/bin/tmdgfiles"
}
```

- >> Construire le paquet
```bash
makepkg -si
```

---

### 📦 4. AppImage (*optionnel*)
```bash
cargo install appimage-builder
appimage-builder --recipe AppImageBuilder.yml
```

---

✔️ Packaging terminé

---
