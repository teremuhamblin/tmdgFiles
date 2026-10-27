###### UNINSTALL.md >> markdown
# 🟩 tmdgFiles
### 🧹 Désinstallation standard
- Si tmdgFiles a été installé via compilation locale :

```bash
rm -f target/release/tmdgFiles
rm -f target/debug/tmdgFiles
```

---

### 🧹 Désinstallation globale (*/usr/local/bin*)
- Si vous avez installé le binaire dans /usr/local/bin :

```bash
sudo rm -f /usr/local/bin/tmdgfiles
```

---

### 🧹 Désinstallation via Cargo
- Si installé via :

```bash
cargo install --path .
```

- Alors désinstaller :

```bash
cargo uninstall tmdgfiles
```

---

### 🧹 Nettoyage complet du projet
```bash
cargo clean
```

---

### 🧹 Suppression du dossier du projet
```bash
rm -rf tmdgFiles
```

---

### ✔️ Désinstallation terminée

---
