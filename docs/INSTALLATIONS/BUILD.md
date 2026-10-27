###### BUILD.md >> markdown 
# 🟦 tmdgFiles
- — ***Compilation avancée***

### 🔧 Compilation standard
```bash
cargo build --release
```

### 🚀 Compilation optimisée CPU (*native*)
- Optimise le binaire pour votre processeur :
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### 🧱 Compilation Debug
```bash
cargo build
./target/debug/tmdgFiles
```

### 🧪 Tests unitaires
```bash
cargo test
```

### 📦 Génération du binaire final
- Le binaire se trouve dans :
```text
target/release/tmdgFiles
```

---

### 🧩 Compilation statique (*avancé*)
- Uniquement si votre distribution supporte musl + GTK4 statique
```bash
sudo apt install musl-tools
cargo build --release --target x86_64-unknown-linux-musl
```

### 🧰 Variables utiles
```text
- CARGOTARGETDIR=... → changer le dossier de build  
- RUSTFLAGS="..." → optimisation avancée  
- cargo clean → nettoyage complet  
```

✔️ ***Compilation avancée terminée***

---
