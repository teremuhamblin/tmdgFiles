###### docs/filesystem.md >> markdown 
---

📄 Filesystem
- — tmdgFiles
Version : v1.0.0

Le module Filesystem gère la lecture des dossiers et l’affichage des fichiers.

🎯 Objectif
Fournir une base simple pour :
- Lister les fichiers
- Lire les dossiers
- Préparer les futures actions (copier, déplacer, renommer, supprimer)

🔧 Fonction principale
`rust
pub fn list_files(path: &str) -> Vec<String>
`

📂 Fonctionnement
1. Lecture du dossier via std::fs::read_dir
2. Extraction des noms de fichiers
3. Retour d’un vecteur de chaînes

🏁 Objectif
Offrir une couche filesystem propre, stable et extensible.
`

---
