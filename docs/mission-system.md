###### docs/mission-system.md >> markdown 

---

📄 Mission System
— tmdgFiles
- Version : v1.0.0

Le système de “Mission” est une fonctionnalité unique de tmdgFiles.  
Il permet de regrouper plusieurs actions sur des fichiers dans une seule opération automatisée.

---

🎯 Objectif
Simplifier les actions répétitives ou complexes en les regroupant dans une mission unique.

Exemples :
```text
- Nettoyer un dossier (supprimer, déplacer, renommer)
- Préparer une archive projet
- Organiser des fichiers téléchargés
- Appliquer des tags en masse
```

---

🧩 Structure d’une Mission

Une mission contient :

- Nom  
- Liste de fichiers  
- Actions à exécuter (dans les futures versions)  
- Journalisation des opérations

---

🗂️ Exemple de structure interne (Rust)

```rust
pub struct Mission {
    pub name: String,
    pub files: Vec<String>,
}
```

---

🔄 Cycle de vie d’une Mission
```text
1. Création  
   L’utilisateur crée une mission avec un nom.

2. Ajout de fichiers  
   Les fichiers sont ajoutés à la mission.

3. Exécution  
   Les actions sont appliquées à tous les fichiers.

4. Journalisation  
   Un log simple enregistre les opérations effectuées.
```

---

🛠️ Fonctionnalités prévues (v2.0.0)
```text
- Actions multiples : copier, déplacer, renommer, supprimer  
- Actions personnalisées (scripts)  
- Missions enregistrées dans un fichier JSON  
- Interface graphique dédiée  
- Mode “Batch Ops”  
- Export / import de missions
```

---

🏁 Conclusion

- Le système de Mission est conçu pour rendre *tmdgFiles* plus puissant que les gestionnaires classiques, en permettant d’automatiser des tâches complexes de manière simple et intuitive.

---
