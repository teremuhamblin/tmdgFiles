###### docs/config.md >> markdown 

---

📄 Configuration
- — tmdgFiles
- Version : v1.0.0

La configuration de tmdgFiles est volontairement simple afin de garder le projet léger et facile à maintenir.

🎯 Objectif
Définir quelques paramètres globaux utilisés par l’application.

---

⚙️ Fichier principal
>> src/config.rs

---

🔧 Paramètres actuels
```rust
pub const DEFAULT_PATH: &str = "/home";
```

---

📌 Usage
```text
- Définir le dossier d’ouverture par défaut
- Centraliser les constantes globales
- Préparer l’ajout futur de préférences utilisateur
```

🏁 Conclusion
Un module minimal, clair, prêt à évoluer dans les prochaines versions.

---
