# tmdgFiles — ROADMAP
Cycle de développement : v1.0.0 → v1.0.9  
Objectif : construire progressivement la base technique du file manager Rust + GTK4  
Style : moderne, structuré, cases cochées ✔️

---

## [1.0.0] — Base initiale
### ✔️ Objectifs atteints
- ✔️ Création du projet tmdgFiles
- ✔️ Mise en place du moteur Rust + GTK4
- ✔️ Première architecture : window.rs, filesystem.rs
- ✔️ README.md initial

### ⏳ Objectifs prévus
- ☐ Structurer les modules internes
- ☐ Définir les standards de documentation
- ☐ Préparer la future arborescence du projet

---

## [1.0.1] — Structure du projet
### ✔️ Objectifs atteints
- ✔️ Création des dossiers : src/, docs/, .github/
- ✔️ Ajout de .gitignore & .gitattributes
- ✔️ Organisation interne des modules Rust

### ⏳ Objectifs prévus
- ☐ Mise en place des conventions de commit
- ☐ Définition des règles CODEOWNERS
- ☐ Préparation du pack de documentation

---

## [1.0.2] — Documentation Pack
### ✔️ Objectifs atteints
- ✔️ Création de docs/architecture.md
- ✔️ Création de docs/ui-design.md
- ✔️ Création de docs/filesystem.md
- ✔️ Modernisation du README.md

### ⏳ Objectifs prévus
- ☐ Ajouter des diagrammes Mermaid
- ☐ Définir les standards de contribution
- ☐ Préparer les guides avancés (tags, missions)

---

## [1.0.3] — Core Stabilisation
### ✔️ Objectifs atteints
- ✔️ Gestion centralisée des erreurs
- ✔️ Logs structurés (info, warn, error)
- ✔️ Stabilisation du module config.rs

### ⏳ Objectifs prévus
- ☐ Ajouter un mode debug
- ☐ Ajouter un mode verbose
- ☐ Préparer la refonte du filesystem

---

## [1.0.4] — UI Architecture
### ✔️ Objectifs atteints
- ✔️ Séparation claire : window.rs, sidebar.rs, file_list.rs
- ✔️ Gestion propre des signaux GTK4
- ✔️ Documentation ui-design.md

### ⏳ Objectifs prévus
- ☐ Ajouter un système de thèmes
- ☐ Ajouter des animations fluides
- ☐ Préparer la modernisation de la sidebar

---

## [1.0.5] — Tags System v1.0
### ✔️ Objectifs atteints
- ✔️ Création du système de tags JSON (tags.json)
- ✔️ API interne pour ajouter/supprimer des tags
- ✔️ Documentation complète dans docs/tags-system.md

### ⏳ Objectifs prévus
- ☐ Recherche avancée par tags
- ☐ Filtrage multi-tags
- ☐ Synchronisation des tags entre sessions

---

## [1.0.6] — Mission System v1.0
### ✔️ Objectifs atteints
- ✔️ Missions : copier, déplacer, supprimer, renommer
- ✔️ Interface dédiée dans docs/mission-system.md
- ✔️ Validation avant exécution

### ⏳ Objectifs prévus
- ☐ Missions parallèles (multi-thread)
- ☐ Historique des missions
- ☐ Annulation en cours d’exécution

---

## [1.0.7] — Filesystem Core Upgrade
### ✔️ Objectifs atteints
- ✔️ Nouveau moteur filesystem.rs plus rapide
- ✔️ Support des opérations par lot (batch ops)
- ✔️ Détection automatique des permissions

### ⏳ Objectifs prévus
- ☐ Préchargement intelligent des métadonnées
- ☐ Mode “scan rapide”
- ☐ Optimisation des accès à /mnt

---

## [1.0.8] — UI & UX Boost
### ✔️ Objectifs atteints
- ✔️ Nouveau thème clair + sombre
- ✔️ Icônes vectorielles GTK4
- ✔️ Animations d’ouverture de dossier

### ⏳ Objectifs prévus
- ☐ Mode compact
- ☐ Mode focus (sans sidebar)
- ☐ Système de préférences utilisateur

---

## [1.0.9] — Stabilisation finale v1.0.x
### ✔️ Objectifs atteints
- ✔️ Optimisation du système de tags
- ✔️ Amélioration du Mission System
- ✔️ Cache interne pour accélérer la navigation
- ✔️ UI plus fluide
- ✔️ Filesystem stabilisé

### ⏳ Objectifs prévus
- ☐ Préparation de la roadmap v2.0.0
- ☐ Nettoyage du code obsolète
- ☐ Finalisation des documents techniques

---
