###### docs/architecture.md >> markdown

📐 Architecture
- — tmdgFiles
- Version : v1.0.0

**tmdgFiles** est *un gestionnaire de fichiers léger et moderne* construit en ***Rust avec une interface GTK4***.
L’architecture est pensée pour être simple, modulaire, et extensible.

---

🧱 1. Vue d’ensemble

```text
tmdgFiles
 ├── src/
 │    ├── main.rs
 │    ├── ui/
 │    ├── core/
 │    └── config.rs
 ├── assets/
 └── docs/
```

L’application est divisée en trois couches principales :
- UI Layer (GTK4) — Interface graphique, fenêtres, sidebar, liste de fichiers  
- Core Layer (Rust) — Logique métier : fichiers, tags, missions  
- Config Layer — Paramètres globaux

---

🖥️ 2. UI Layer (GTK4)
- Dossier : **~/src/ui/**
```text
>> Objectif
- Gérer l’affichage, les interactions utilisateur et la structure visuelle.

>> Modules
- window.rs
   - Crée la fenêtre principale  
   - Assemble la sidebar et la liste des fichiers  
   - Initialise l’interface GTK4

>> sidebar.rs
   - Affiche les sections : Home, Documents, Téléchargements, Favoris  
   - Sert de navigation rapide

>> file_list.rs
   - Affiche les fichiers du dossier courant  
   - Sera étendu pour afficher : nom, type, taille, tags
```

---

⚙️ 3. Core Layer (Rust)
- Dossier : **~/src/core/**
```text
>> Objectif
- Gérer la logique interne du gestionnaire de fichiers.

>> Modules
- filesystem.rs
   - Lecture des dossiers  
   - Récupération des fichiers  
   - Opérations futures : copier, déplacer, renommer, supprimer

>> tags.rs
   - Système de tags basé sur JSON  
   - Ajout / suppression de tags  
   - Filtrage par tags

>> missions.rs
   - Concept de “Mission” (ensemble d’actions sur des fichiers)  
   - Stockage des fichiers associés  
   - Exécution future des actions en batch
```

---

⚙️ 4. Config Layer

- Fichier : **~/src/config.rs**

>> Objectif
- Définir les constantes globales du projet.

Exemple :
```text
pub const DEFAULT_PATH: &str = "/home";
```

---

🔄 5. Flux d’exécution

```text
main.rs
   ↓
Application GTK4
   ↓
window.rs → sidebar.rs + file_list.rs
   ↓
core/filesystem.rs (liste des fichiers)
   ↓
Affichage dans l’UI
```

---

🧩 6. Extensibilité prévue

La structure est pensée pour accueillir facilement :
```text
- Actions avancées (copier, déplacer, renommer, supprimer)
- Système de tags complet
- Vue Mission automatisée
- Mode compact “Ops”
- Thèmes visuels
- Plugins externes
- Support multi‑fenêtres
```

---

🏁 7. Conclusion

- L’architecture de tmdgFiles est volontairement simple, claire, et modulaire, permettant une évolution rapide vers les futures versions tout en restant facile à maintenir.

---
