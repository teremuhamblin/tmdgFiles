# tmdgFiles — CHANGELOG
Versionnage : v1.0.x  
Style : clair, structuré, cases à cocher remplies ✔️  
Technos : Rust + GTK4 + Linux Desktop

---

## [1.0.9] — Stabilisation finale v1.0.x
### ✔️ Ajouté
- ✔️ Optimisation du système de tags (meilleure indexation)
- ✔️ Amélioration du Mission System (exécution séquentielle stable)
- ✔️ Ajout d’un cache interne pour accélérer la navigation

### ✔️ Modifié
- ✔️ UI : transitions plus fluides entre panneaux
- ✔️ Filesystem : réduction des appels système redondants

### ✔️ Corrigé
- ✔️ Correction d’un freeze lors du renommage massif
- ✔️ Correction d’un bug d’affichage dans le panneau latéral

---

## [1.0.8] — UI & UX Boost
### ✔️ Ajouté
- ✔️ Nouveau thème clair + sombre
- ✔️ Icônes vectorielles optimisées GTK4
- ✔️ Animation d’ouverture de dossier

### ✔️ Modifié
- ✔️ Sidebar plus compacte
- ✔️ Amélioration du layout responsive

### ✔️ Corrigé
- ✔️ Correction d’un problème de focus clavier

---

## [1.0.7] — Filesystem Core Upgrade
### ✔️ Ajouté
- ✔️ Nouveau module filesystem.rs plus rapide
- ✔️ Support des opérations par lot (batch ops)
- ✔️ Détection automatique des permissions

### ✔️ Modifié
- ✔️ Refonte du système de logs internes

### ✔️ Corrigé
- ✔️ Correction d’un crash lors de l’accès à /mnt

---

## [1.0.6] — Mission System v1.0
### ✔️ Ajouté
- ✔️ Missions : copier, déplacer, supprimer, renommer
- ✔️ Interface dédiée dans docs/mission-system.md
- ✔️ Système de validation avant exécution

### ✔️ Modifié
- ✔️ Amélioration du moteur d’exécution des tâches

### ✔️ Corrigé
- ✔️ Correction d’un bug de progression non affichée

---

## [1.0.5] — Tags System v1.0
### ✔️ Ajouté
- ✔️ Système de tags JSON (tags.json)
- ✔️ API interne pour ajouter/supprimer des tags
- ✔️ Documentation complète dans docs/tags-system.md

### ✔️ Modifié
- ✔️ UI : affichage des tags dans la liste de fichiers

### ✔️ Corrigé
- ✔️ Correction d’un bug de sauvegarde des tags

---

## [1.0.4] — UI Architecture
### ✔️ Ajouté
- ✔️ Séparation claire : window.rs, sidebar.rs, file_list.rs
- ✔️ Gestion propre des signaux GTK4
- ✔️ Documentation ui-design.md

### ✔️ Modifié
- ✔️ Refonte du système d’événements

### ✔️ Corrigé
- ✔️ Correction d’un bug de double-clic

---

## [1.0.3] — Core Stabilisation
### ✔️ Ajouté
- ✔️ Gestion des erreurs centralisée
- ✔️ Logs structurés (info, warn, error)

### ✔️ Modifié
- ✔️ Amélioration du module config.rs

### ✔️ Corrigé
- ✔️ Correction d’un crash lors de l’ouverture de gros dossiers

---

## [1.0.2] — Documentation Pack
### ✔️ Ajouté
- ✔️ docs/architecture.md
- ✔️ docs/ui-design.md
- ✔️ docs/filesystem.md

### ✔️ Modifié
- ✔️ README.md modernisé

### ✔️ Corrigé
- ✔️ Correction de liens internes

---

## [1.0.1] — Structure du projet
### ✔️ Ajouté
- ✔️ Arborescence complète : src/, docs/, .github/
- ✔️ Mise en place de .gitignore & .gitattributes

### ✔️ Modifié
- ✔️ Organisation interne des modules Rust

### ✔️ Corrigé
- ✔️ Correction de warnings Rust

---

## [1.0.0] — Base initiale
### ✔️ Ajouté
- ✔️ Création du projet tmdgFiles
- ✔️ Setup Rust + GTK4
- ✔️ Première version de window.rs & filesystem.rs
- ✔️ README.md initial

### ✔️ Modifié
- ✔️ Ajustements mineurs du layout

### ✔️ Corrigé
- ✔️ Correction de compilation initiale
