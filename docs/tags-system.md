###### docs/tags-system.md >> markdown 

---

📄 Tags System
- — tmdgFiles
- Version : v1.0.0

Le système de tags permet d’ajouter des étiquettes aux fichiers pour faciliter l’organisation, la recherche et le filtrage.

---

🎯 Objectif
Offrir une organisation plus flexible que les dossiers traditionnels.
```text
Exemples de tags :
- projet
- client
- archive
- urgent
- perso
```

---

🧩 Structure interne

- Les tags sont stockés dans un fichier JSON local.

```rust
pub struct TagStore {
    pub tags: HashMap<String, Vec<String>>,
}
```

- Format JSON prévu :

```json
{
  "mon_fichier.txt": ["projet", "urgent"],
  "image.png": ["archive"]
}
```

---

🔧 Fonctionnalités v1.0.0

- Ajouter un tag à un fichier  
- Supprimer un tag  
- Lister les tags d’un fichier  
- Filtrer les fichiers par tag  
- Stockage local simple (JSON)

---

🔄 Cycle de fonctionnement

1. Lecture du fichier JSON  
2. Modification des tags  
3. Sauvegarde automatique  
4. Affichage dans l’interface  
5. Filtrage dynamique

---

🛠️ Fonctionnalités prévues (v2.0.0)
```text
- Interface graphique pour gérer les tags  
- Couleurs de tags  
- Tags persistants dans la barre latérale  
- Recherche avancée par tags  
- Tags multiples sur sélection de fichiers  
- Synchronisation des tags entre missions
```

---

🏁 Conclusion

- Le système de tags apporte une couche d’organisation moderne et flexible, permettant à tmdgFiles de dépasser les gestionnaires de fichiers traditionnels.

---
