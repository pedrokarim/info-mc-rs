# Plan des nouveaux outils MCInfo

## Ajouts à des outils existants

### MOTD Editor → + Tellraw / JSON Text Generator
- Ajouter un mode `/tellraw`, `/title`, `/actionbar`
- Même moteur de rendu texte MC, même palette de couleurs
- Génération de la commande complète copiable
- Preview in-game simulée (chat, title screen, actionbar)

### NBT Viewer → + Schematic Viewer avancé
- Support `.schem` (Sponge/WorldEdit) et `.litematic` (Litematica)
- Rendu 3D avec textures de blocs (pas juste couleurs)
- Navigation caméra dans la structure
- Infos : dimensions, nombre de blocs, palette utilisée

---

## Nouveaux outils standalone

### Coordinate Calculator
- Conversion Overworld ↔ Nether (÷8 / ×8)
- Finder de chunk (÷16) et region (÷512)
- Distance entre deux points (2D et 3D)
- Spawn chunk visualizer
- Complexité : **faible** (pur frontend, zéro backend)

### Banner Designer
- Composer un pattern de bannière couche par couche (max 6 layers)
- Liste des 39 patterns MC disponibles
- Palette des 16 couleurs MC
- Preview 2D en temps réel
- Génération commande `/give`
- Complexité : **moyenne** (rendu canvas 2D, données patterns)

### Firework Designer
- Sélection forme (small ball, large ball, star, creeper, burst)
- Couleurs / fade colors
- Effets (trail, twinkle)
- Durée de vol (1-3)
- Preview visuel de l'explosion
- Génération commande `/give`
- Complexité : **moyenne** (preview animée optionnelle)

### Enchantment Calculator
- Sélection d'un item → liste des enchantements compatibles
- Ajout d'enchantements sur plusieurs livres/items
- Calcul du coût XP par étape sur l'enclume
- Calcul de l'ordre optimal (minimiser le coût total)
- Gestion "Too Expensive!" (cap 39 niveaux en survie)
- Complexité : **moyenne** (algo d'optimisation combinatoire)

### Command Generator
- Générateur pour `/summon`, `/give`, `/fill`, `/setblock`, `/particle`
- Autocomplete sur les IDs (blocs, items, entités, enchantements)
- Éditeur de NBT visuel pour les tags d'entités/items
- Preview de la commande en temps réel
- Complexité : **élevée** (beaucoup de data MC à maintenir, UI riche)

---

## Ordre de priorité suggéré

| #  | Outil                  | Type        | Effort | Valeur |
|----|------------------------|-------------|--------|--------|
| 1  | Coordinate Calculator  | Nouveau     | Faible | Haute  |
| 2  | Tellraw Generator      | Extension   | Faible | Haute  |
| 3  | Banner Designer        | Nouveau     | Moyen  | Haute  |
| 4  | Schematic Viewer       | Extension   | Moyen  | Haute  |
| 5  | Firework Designer      | Nouveau     | Moyen  | Moyenne|
| 6  | Enchantment Calculator | Nouveau     | Moyen  | Haute  |
| 7  | Command Generator      | Nouveau     | Élevé  | Haute  |
