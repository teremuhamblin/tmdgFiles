###### README.md >> markdown

---

1️⃣ tmdgFiles
- v1.0.0

**tmdgFiles** est un *gestionnaire de fichiers* moderne, léger et unique, inspiré de *Nemo/Nautilus* mais conçu pour être simple, rapide et extensible.

🚀 Fonctionnalités principales
```text
- Navigation fluide dans les dossiers
- Affichage des fichiers (nom, type, taille)
- Actions essentielles : ouvrir, copier, déplacer, renommer, supprimer
- Historique de navigation
- Barre latérale (Home, Documents, Téléchargements, Favoris)
- Système de tags (JSON)
- Mode compact “Ops”
- Vue “Mission” pour automatiser des actions sur plusieurs fichiers
```

🧱 Architecture principale du projet
```text
src/
 ├── main.rs
 ├── ui/
 │    ├── window.rs
 │    ├── sidebar.rs
 │    └── file_list.rs
 ├── core/
 │    ├── filesystem.rs
 │    ├── tags.rs
 │    └── missions.rs
 └── config.rs
```

📦 Installation
- Dépendances Linux
   - **sudo apt install libgtk-4-dev**

- Compilation
```text
cargo build
cargo run
```

🗺️ Roadmap
>> Voir docs/roadmap.md.

📄 Licence
>> Projet sous licence Unlicense (liberté totale).

🤝 Contribution
>> Pull Requests bienvenues.

---
