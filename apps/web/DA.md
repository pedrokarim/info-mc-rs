# DA MCInfo (v2 - Chunklock Direction)

## 1) Intent

DA originale inspiree de portails Minecraft type Chunklock:

- Header compact style arcade (onglets + CTA serveur)
- Hero media impactant avec vignette (look "gaming homepage")
- Panneaux data lisibles et coherents (server, skin, player)
- Palette noir/bleu + accent or pour les actions primaires

## 2) Visual DNA

- Univers: game portal, pas dashboard bureautique
- Contraste: fond noir profond + surfaces bleu nuit
- Accent: bleu froid pour navigation, or chaud pour CTA
- Tone: premium mais direct, orientee action et lookup rapide

## 3) Design Tokens

Source unique: `apps/web/src/app.css` (`:root`).

- Fonds: `--bg-0`, `--bg-1`, `--bg-2`
- Surfaces: `--surface-0`, `--surface-1`, `--surface-soft`
- Texte: `--text-0`, `--text-1`, `--text-2`
- Bleu UI: `--blue-0`, `--blue-1`
- Or CTA: `--gold-0`, `--gold-1`
- Etats: `--ok`, `--danger`
- Structure: `--radius-*`, `--shadow-*`, `--layout-width`

## 4) Typography

- Police principale: `Chakra Petch` + `Rajdhani`
- Titres: gros, denses, tracking legerement positif
- Labels/meta: uppercase fin + contraste secondaire

## 5) Component Contract

- Header: `.topbar`, `.topnav`, `.nav-link`, `.nav-link.cta`
- Hero: `.hero`, `.hero-copy`, `.hero-media`, `.hero-overlay`
- Surface standard: `.surface`
- Form controls: `.input`, `.select`, `.mode-switch`
- Actions: `.btn-primary`, `.btn-secondary`, `.chip`
- Data cards: `.card`, `.card-head`, `.card-body`, `.kv`
- Status badges: `.status.neutral|online|offline|loading`

## 6) Page Grammar

- Etage 1: hero (message + media)
- Etage 2: bloc interaction (lookup / filtres)
- Etage 3: cartes data (stats, MOTD, renders, meta)
- Footer court, meme ton visuel que le header

## 7) Route Mapping

- `/`: hero accueil + lookup unifie + spotlight
- `/server/[address]`: hero serveur + snapshot + MOTD visual
- `/skins`: hero skin + recherche + galerie
- `/player/[username]`: hero joueur + details + rendus

## 8) Rules

- Pas de nouvelle couleur hors tokens
- Tous les CTA primaires doivent rester sur le style `.btn-primary` ou `.nav-link.cta`
- Toute nouvelle section commence par `surface` ou `hero` (pas de bloc non cadre)
- Les backgrounds images doivent rester Minecraft voxel lisibles
