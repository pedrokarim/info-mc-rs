# Inventaire Images - MCInfo Web

Objectif: centraliser toutes les images générées pour la DA et suivre leur intégration.

## Règles rapides

- Emplacement: `apps/web/static/images/...`
- URL publique: `/images/...`
- Formats recommandés:
  - `webp` (prioritaire)
  - `png` (si transparence/asset UI)
  - `jpg` (photos/rendus lourds)
- Nommage conseillé: `zone-sujet-variant.ext`
  - Ex: `hero-overworld-v01.webp`

## Statuts

- `todo`: à générer
- `done`: générée
- `integrated`: utilisée dans le front
- `replace`: version à améliorer/remplacer
- `wired`: référencée dans le CSS/front, en attente du fichier final

## Style Minecraft requis (non négociable)

Objectif: éviter les visuels génériques fantasy/sci-fi qui n'ont aucun rapport avec Minecraft.

### Ce qu'il faut absolument

- Univers voxel/blocky lisible (formes cubiques, matériaux en blocs).
- Éléments iconiques Minecraft: herbe, pierre, bois, torches, biomes, structures cubiques.
- Lumière "jeu vidéo" claire, avec saturation maîtrisée (pas photo-réaliste).
- Silhouettes nettes et lisibles même en petite taille.
- Ambiance "serveur de jeu": vivant, communautaire, orienté gameplay.

### Ce qu'il faut éviter absolument

- Personnages réalistes/humains non voxel.
- Architecture réaliste moderne sans style bloc.
- Concept art fantasy générique sans codes Minecraft.
- Overlays trop chargés qui rendent le texte illisible.
- Images sombres/floues sans point focal.

### Mots-clés de génération conseillés

- `minecraft-inspired voxel world`
- `blocky terrain`, `pixelated textures`, `cubic trees`
- `game server hub`, `adventure survival atmosphere`
- `cinematic but stylized`, `high readability`, `clean focal point`

### Mots-clés à exclure

- `photorealistic`, `real human`, `ultra realistic face`
- `cyberpunk city`, `space station`, `modern office`
- `anime portrait`, `oil painting`, `baroque`

---

## Hero (`/images/hero/`)

| Fichier | Usage | Dimension cible | Format | Statut | Page(s) | Notes |
|---|---|---:|---|---|---|---|
| hero-main-v01.png | Hero principal home | 1920x1080 | png | integrated | `/` | branche dans `app.css` (`.hero-media-home`) |
| hero-main-v02.png | Variante A/B test | 1920x1080 | png | done | `/` | dispo pour A/B test |
| hero-mobile-v01.png | Hero mobile | 1080x1350 | png | done | `/` | même scène, recadrage vertical lisible |

## Servers (`/images/servers/`)

| Fichier | Usage | Dimension cible | Format | Statut | Page(s) | Notes |
|---|---|---:|---|---|---|---|
| server-card-bg-v01.png | Fond card serveur | 1200x700 | png | integrated | `/server/[address]` | branche dans `app.css` (`.hero-media-server`) |
| motd-frame-v01.png | Cadre visuel MOTD | 1600x400 | png | todo | `/server/[address]` | overlay léger |
| server-empty-state-v01.png | Empty state serveur | 1200x700 | png | done | `/server/[address]` | fallback prêt |

## Skins (`/images/skins/`)

| Fichier | Usage | Dimension cible | Format | Statut | Page(s) | Notes |
|---|---|---:|---|---|---|---|
| skin-gallery-bg-v01.png | Fond section galerie | 1600x900 | png | integrated | `/skins` | branche dans `app.css` (`.hero-media-skins`) |
| skin-detail-bg-v01.png | Fond section détail skin | 1600x900 | png | integrated | `/player/[username]` | branche dans `app.css` (`.hero-media-player`) |
| skin-placeholder-head-v01.png | Placeholder head | 256x256 | png | done | `/skins`, `/player/[username]` | transparent |
| skin-placeholder-full-v01.png | Placeholder full body | 256x512 | png | done | `/player/[username]` | transparent |

## SEO / Open Graph (`/images/`)

| Fichier | Usage | Dimension cible | Format | Statut | Page(s) | Notes |
|---|---|---:|---|---|---|---|
| og-default.png | Image OG par défaut (partage réseaux sociaux) | 1200x630 | png | todo | toutes | référencée dans `SEO.svelte` |

## UI (`/images/ui/`)

| Fichier | Usage | Dimension cible | Format | Statut | Page(s) | Notes |
|---|---|---:|---|---|---|---|
| noise-overlay-v01.png | Grain léger global | 1024x1024 | png | todo | global | repeatable |
| glow-orb-blue-v01.png | Glow décoratif bleu | 800x800 | png | todo | global | alpha douce |
| glow-orb-gold-v01.png | Glow décoratif or | 800x800 | png | todo | global | alpha douce |
| divider-tech-v01.svg | Séparateur section | 1600x80 | svg | todo | global | scalable |

---

## Images actuellement présentes

À compléter automatiquement ou manuellement au fil des ajouts.

- `static/images/hero/hero-main-v01.png` — `integrated`
- `static/images/hero/hero-main-v02.png` — `done`
- `static/images/hero/hero-mobile-v01.png` — `done`
- `static/images/servers/server-card-bg-v01.png` — `integrated`
- `static/images/servers/server-empty-state-v01.png` — `done`
- `static/images/skins/skin-gallery-bg-v01.png` — `integrated`
- `static/images/skins/skin-detail-bg-v01.png` — `integrated`
- `static/images/skins/skin-placeholder-head-v01.png` — `done`
- `static/images/skins/skin-placeholder-full-v01.png` — `done`
- `static/images/ui/mc_dirt.jpg`
  - Usage: fond du bloc MOTD
  - Statut: `integrated`

---

## Checklist qualité avant intégration

- [ ] Poids optimisé (hero < 500KB si possible)
- [ ] Version mobile prévue si nécessaire
- [ ] Contraste texte/fond vérifié
- [ ] Cohérence DA (même palette, même lumière)
- [ ] Le visuel est clairement identifiable comme Minecraft (test 2 secondes)
- [ ] Nom de fichier propre et versionné (`v01`, `v02`, ...)
- [ ] Ajouté dans la section ci-dessus avec statut mis à jour

---

## Prompts prets (copier-coller)

Utiliser ces prompts directement avec la taille indiquee.

### hero-main-v01.png

- Taille: `1920x1080` (16:9)

```txt
PROMPT:
Wide cinematic hero scene of a minecraft-inspired voxel world, bright daytime, blocky mountains, cubic trees, server spawn hub, clear path leading to central plaza, strong focal point center-right, clean empty area on left for UI text, stylized game art, high readability, no text, no logo, no watermark.

NEGATIVE:
photorealistic, real humans, cyberpunk city, sci-fi spaceship, anime face, blurry, low contrast, cluttered composition, text, logo, watermark.
```
### skin-placeholder-full-v01.png

- Taille: `256x512` (1:2, fond transparent)

```txt
PROMPT:
Minimal minecraft-inspired voxel full-body placeholder silhouette, centered, clean edges, transparent alpha background, UI fallback asset style, no text, no logo, no watermark.

NEGATIVE:
photoreal person, environment background, clutter, text, logo, watermark.
```
### hero-main-v02.png

- Taille: `1920x1080` (16:9)

```txt
PROMPT:
Epic sunset minecraft-inspired voxel landscape, orange-blue sky, blocky fortress in distance, torches glow, survival server atmosphere, layered depth, clean left area for headline, premium gaming website hero look, highly readable, no text, no logo, no watermark.

NEGATIVE:
photoreal faces, modern city, futuristic weapons, muddy colors, heavy fog, visual noise, text, logo, watermark.
```

### hero-mobile-v01.png

- Taille: `1080x1350` (4:5)

```txt
PROMPT:
Vertical hero artwork of a minecraft-inspired voxel world, blocky terrain, central spawn hub, clear foreground path, mobile-friendly centered framing, stylized lighting, high readability for UI overlay, no text, no logo, no watermark.

NEGATIVE:
horizontal composition, tiny unreadable details, realistic humans, cyberpunk, blur, low contrast, text, logo, watermark.
```

### server-card-bg-v01.png

- Taille: `1200x700` (~12:7)

```txt
PROMPT:
Minecraft-inspired voxel server hub background for info card, subtle depth, dark-blue mood, left 45% intentionally clean for text overlay, right side with soft blocky structures and warm lights, minimal noise, premium gaming UI background, no text, no logo, no watermark.

NEGATIVE:
busy full-frame details, no empty space, photoreal buildings, realistic humans, overbloom, text, logo, watermark.
```

### server-empty-state-v01.png

- Taille: `1200x700` (~12:7)

```txt
PROMPT:
Calm twilight minecraft-inspired voxel landscape for offline/empty state, blocky hills, sparse warm lights, readable center area for status message, elegant game UI background, no text, no logo, no watermark.

NEGATIVE:
horror style, black crush, photorealism, clutter, heavy particles, text, logo, watermark.
```

### skin-gallery-bg-v01.png

- Taille: `1600x900` (16:9)

```txt
PROMPT:
Subtle minecraft-inspired voxel pattern background for skin gallery, block textures and soft geometric depth, low visual noise, medium contrast, designed to keep thumbnails readable, premium gaming site style, no text, no logo, no watermark.

NEGATIVE:
strong central subject, character portrait, photorealism, high clutter, distracting highlights, text, logo, watermark.
```

### skin-detail-bg-v01.png

- Taille: `1600x900` (16:9)

```txt
PROMPT:
Focused minecraft-inspired voxel environment for player skin detail page, central platform/pedestal vibe, subtle dramatic light, clean side zones for profile panels, high readability, premium game UI aesthetic, no text, no logo, no watermark.

NEGATIVE:
realistic human body, sci-fi city, cluttered center, over-detailed noise, low contrast, text, logo, watermark.
```

### og-default.png

- Taille: `1200x630` (ratio OG standard)

```txt
PROMPT:
Wide banner for social media sharing (Open Graph), minecraft-inspired voxel world, panoramic blocky landscape with iconic elements (grass blocks, torches, cubic trees, stone fortress), bright vibrant colors, strong center composition, large clean area center for branding overlay "MCInfo", premium gaming website feel, high contrast, sharp edges, no text, no logo, no watermark.

NEGATIVE:
photorealistic, real humans, anime, dark moody, low contrast, blurry, busy composition with no focal point, vertical framing, text, logo, watermark.
```

> Note: ajouter le texte "MCInfo" + tagline en post-prod (Figma/Canva) sur la zone clean centrale.

### skin-placeholder-head-v01.png

- Taille: `256x256` (1:1, fond transparent)

```txt
PROMPT:
Minimal minecraft-inspired voxel placeholder icon, neutral blocky head silhouette, centered, clean edges, transparent alpha background, no text, no logo, no watermark.

NEGATIVE:
photoreal face, background scene, complex details, text, logo, watermark.
```

### skin-placeholder-full-v01.png

- Taille: `256x512` (1:2, fond transparent)

```txt
PROMPT:
Minimal minecraft-inspired voxel full-body placeholder silhouette, centered, clean edges, transparent alpha background, UI fallback asset style, no text, no logo, no watermark.

NEGATIVE:
photoreal person, environment background, clutter, text, logo, watermark.
```
