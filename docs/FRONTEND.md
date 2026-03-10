# MCInfo-RS - Specifications Frontend

Interface web gaming-style pour visualiser les informations serveurs MC,
profils joueurs, et rendus 3D de skins.

---

## Stack Frontend

| Composant | Technologie | Justification |
|-----------|------------|---------------|
| Framework | **SvelteKit** (choix recommande) | SSR natif, performant, DX excellent |
| 3D Rendering | **Three.js** via threlte (Svelte) | Rendu WebGL des skins MC |
| Animations | **GSAP** ou **Motion One** | Animations fluides, scroll-triggered |
| Charts | **Chart.js** ou **Recharts** (si React) | Graphiques historique joueurs |
| Styling | **Tailwind CSS** + custom | Utilitaire + theme gaming custom |
| Icons | **Lucide** | Icons clean, coherentes |

---

## Pages & Routes

```
/                           -> Page d'accueil (hero + search)
/server/{address}           -> Resultats lookup serveur
/server/{address}/history   -> Historique detaille serveur
/player/{username}          -> Profil joueur + skin 3D
/player/{username}/skins    -> Historique des skins
/skins                      -> Galerie de skins
/about                      -> A propos du projet
/api                        -> Documentation API (interactive)
```

---

## Design System

### Inspirations visuelles capturees

#### Origin Realms (ref-originrealms.png)
- **Hero immersif** : Grande banniere avec scene MC rendue (monde 3D coloree)
- **Theme tres dark** : Fond noir pur (#000000) avec elements en or/jaune (#FFA500)
- **Navbar** : Minimaliste, transparente sur le hero, items: Home, Blog, Guides, Store
- **Cards actualites** : Fond sombre avec bordures dorees/orange, images a gauche
- **Section "Meet the Team"** : Cards horizontales avec avatars, style carousel
- **CTA** : Boutons avec fond orange/dore, texte bold
- **Footer** : Sombre, liens Discord, partenaires
- **Ambiance** : Immersive, communautaire, chaleureuse

#### NameMC (ref-namemc.png)
- **Layout fonctionnel** : Priorite a la data, pas au style
- **Search bar** : En haut, tabs (Name/UUID/Server/Skin)
- **Skin gallery** : Grille de rendus 3D isometriques des skins
- **Server list** : Tableau avec favicon, nom, IP, joueurs, badge Java/Bedrock
- **Profil joueur** : Rendu 3D du skin a gauche, infos a droite
- **Historique skins** : Timeline avec previews
- **Style** : Propre mais date, on peut faire 100x mieux

### Notre Direction Artistique

On prend le meilleur des deux mondes :
- L'**immersion gaming** d'Origin Realms (dark theme, effets visuels, ambiance)
- Les **features data** de NameMC (skin gallery, server lookup, player profile)
- Un **design moderne** que aucun des deux n'a (glassmorphism, animations smooth)

---

## Palette de Couleurs

```css
:root {
  /* Backgrounds */
  --bg-primary: #0a0a0f;        /* Fond principal, quasi-noir */
  --bg-secondary: #12121a;      /* Cards, sections */
  --bg-tertiary: #1a1a2e;       /* Hover states, inputs */
  --bg-glass: rgba(18, 18, 26, 0.7); /* Glassmorphism */

  /* Primary - Emeraude / Vert MC */
  --primary-400: #34d399;
  --primary-500: #10b981;
  --primary-600: #059669;

  /* Accent - Violet */
  --accent-400: #c084fc;
  --accent-500: #a855f7;
  --accent-600: #9333ea;

  /* Warm accent - Or/Orange (inspire Origin Realms) */
  --warm-400: #fbbf24;
  --warm-500: #f59e0b;
  --warm-600: #d97706;

  /* Texte */
  --text-primary: #f1f5f9;      /* Titres, texte important */
  --text-secondary: #94a3b8;    /* Texte secondaire */
  --text-muted: #475569;        /* Texte desactive */

  /* Etats */
  --online: #22c55e;            /* Serveur en ligne */
  --offline: #ef4444;           /* Serveur hors ligne */
  --unknown: #f59e0b;           /* Statut inconnu */

  /* Glow effects */
  --glow-primary: 0 0 20px rgba(16, 185, 129, 0.3);
  --glow-accent: 0 0 20px rgba(168, 85, 247, 0.3);
}
```

---

## Typographie

```css
/* Headings - Bold, impact gaming */
font-family: 'Inter', 'SF Pro Display', system-ui, sans-serif;
font-weight: 800; /* Extra Bold */
letter-spacing: -0.02em;

/* Body */
font-family: 'Inter', 'SF Pro Text', system-ui, sans-serif;
font-weight: 400;

/* Monospace - IP, UUID, donnees techniques */
font-family: 'JetBrains Mono', 'Fira Code', monospace;

/* MOTD Minecraft */
font-family: 'Minecraft', monospace; /* Font custom pixel si dispo */
```

---

## Composants Principaux

### 1. Hero Section (Page d'accueil)

```
+------------------------------------------------------------------+
|                                                                    |
|  [Particules / blocs MC flottants en arriere-plan]                |
|                                                                    |
|           M C I N F O                                             |
|    Minecraft Server Intelligence                                   |
|                                                                    |
|  +------------------------------------------------------+  [GO]  |
|  |  play.hypixel.net                                    |         |
|  +------------------------------------------------------+         |
|                                                                    |
|    [Server Lookup]   [Player Lookup]   [Skin Gallery]             |
|                                                                    |
+------------------------------------------------------------------+
```

- Background : gradient dark + particules animees (blocs MC pixelises)
- Search bar : grande, centre, border glow au focus (var(--primary-500))
- Tabs sous la search bar pour switcher le type de lookup
- Titre anime avec glow effect

### 2. Server Card (Resultat lookup)

```
+------------------------------------------------------------------+
|  [Favicon 64x64]                                                  |
|                                                                    |
|  play.hypixel.net                            [ONLINE]             |
|  172.65.128.35:25565                     Latency: 34ms            |
|                                                                    |
|  +--------------------------------------------------------------+ |
|  | MOTD rendu avec couleurs MC                                   | |
|  | Hypixel Network [1.8-1.21]                                    | |
|  +--------------------------------------------------------------+ |
|                                                                    |
|  Version: 1.21.4 (Protocol 769)      Players: 42,389 / 200,000  |
|  [========================================-----] 21%              |
|                                                                    |
|  Edition: Java    Type: Bungeecord    SRV: Yes                   |
|                                                                    |
|  Sample joueurs:                                                  |
|  [Avatar] Player1  [Avatar] Player2  [Avatar] Player3  +12 more  |
+------------------------------------------------------------------+
```

- Fond glassmorphism (var(--bg-glass))
- Favicon avec border radius + ombre
- Badge ONLINE/OFFLINE avec couleur d'etat
- Barre de joueurs animee (remplissage progressif)
- MOTD rendu avec les codes couleur MC (&a = vert, &b = cyan, etc.)
- Chaque joueur dans le sample est cliquable -> /player/{name}

### 3. Player Profile Page

```
+------------------------------------------------------------------+
|                                                                    |
|  +------------------+    Username: Dream                          |
|  |                  |    UUID: ec70bcaf-702f-4bb8-b48d-276fa52a780c|
|  |   [Rendu 3D     |                                              |
|  |    du skin       |    Skin: Classic model                      |
|  |    interactif]   |    Cape: Minecon 2016                       |
|  |                  |                                              |
|  |   (drag pour     |    [Download Skin]  [View History]          |
|  |    tourner)      |                                              |
|  |                  |                                              |
|  +------------------+                                              |
|                                                                    |
|  --- Skin History -----------------------------------------------  |
|  [Skin 1]  [Skin 2]  [Skin 3]  [Skin 4]  [Skin 5]              |
|  Jan 2026  Dec 2025  Oct 2025  Aug 2025  Jun 2025               |
|                                                                    |
+------------------------------------------------------------------+
```

- Rendu 3D Three.js du skin a gauche, interactif (drag = rotation)
- Infos textuelles a droite
- Animation idle du personnage (respiration, balancement bras)
- Section historique skins en bas (timeline horizontale)

### 4. MOTD Renderer

Le MOTD MC utilise des codes de formatage (prefixe `§` ou `&`):

```
Codes couleur:
§0 = #000000 (noir)      §8 = #555555 (gris fonce)
§1 = #0000AA (bleu fonce) §9 = #5555FF (bleu)
§2 = #00AA00 (vert fonce) §a = #55FF55 (vert)
§3 = #00AAAA (cyan fonce) §b = #55FFFF (cyan)
§4 = #AA0000 (rouge fonce)§c = #FF5555 (rouge)
§5 = #AA00AA (violet)     §d = #FF55FF (rose)
§6 = #FFAA00 (or)         §e = #FFFF55 (jaune)
§7 = #AAAAAA (gris)       §f = #FFFFFF (blanc)

Codes style:
§l = bold        §o = italic
§n = underline   §m = strikethrough
§k = obfuscated (texte anime aleatoire)
§r = reset
```

Le composant MOTD doit :
- Parser le texte avec codes
- Generer des `<span>` avec les styles CSS correspondants
- Supporter `§k` (obfuscated) avec une animation JS qui change les caracteres
- Supporter le format "Chat Component" JSON en plus du format legacy

### 5. Skin 3D Viewer (Three.js)

Architecture du viewer :

```
SkinViewer (composant principal)
  |-- Scene Three.js
  |   |-- Camera (PerspectiveCamera)
  |   |-- Lights (AmbientLight + DirectionalLight)
  |   |-- PlayerModel (Group)
  |   |   |-- Head (Box 8x8x8)
  |   |   |-- Body (Box 8x12x4)
  |   |   |-- Left Arm (Box 4x12x4 ou 3x12x4 slim)
  |   |   |-- Right Arm (Box 4x12x4 ou 3x12x4 slim)
  |   |   |-- Left Leg (Box 4x12x4)
  |   |   |-- Right Leg (Box 4x12x4)
  |   |   |-- [Overlay layers - memes dimensions mais 0.5px plus grand]
  |   |-- OrbitControls (drag rotation)
  |-- AnimationLoop
  |   |-- IdleAnimation (breathing, arm sway)
  |   |-- WalkAnimation (optionnel)
  |-- TextureLoader
      |-- Charge skin PNG depuis API
      |-- Decoupe UV mapping pour chaque face de chaque partie
```

#### UV Mapping du skin MC (64x64)

Le skin MC est une texture 64x64 pixels avec des zones specifiques :

```
Skin layout (64x64):
+--------+--------+--------+--------+--------+--------+--------+--------+
|        | Head   | Head   |        |        | Head   | Head   |        |
|        | Top    | Bottom |        |        | Top OL | Bot OL |        |
| 0,0    | 8,0    | 16,0   | 24,0   | 32,0   | 40,0   | 48,0   | 56,0   |
+--------+--------+--------+--------+--------+--------+--------+--------+
| Head   | Head   | Head   | Head   | Head   | Head   | Head   | Head   |
| Right  | Front  | Left   | Back   | Right  | Front  | Left   | Back   |
| 0,8    | 8,8    | 16,8   | 24,8   | OL     | OL     | OL     | OL     |
+--------+--------+--------+--------+--------+--------+--------+--------+
|        | Body   | Body   |        |        | Body   | Body   |        |
|        | Top    | Bottom |        |        | Top OL | Bot OL |        |
| 16,16  | 20,16  | 28,16  | 36,16  | 16,32  | 20,32  | 28,32  | 36,32  |
+--------+--------+--------+--------+--------+--------+--------+--------+
| Body   | Body   | Body   | Body   | Body   | Body   | Body   | Body   |
| Right  | Front  | Left   | Back   | Right  | Front  | Left   | Back   |
| 16,20  | 20,20  | 28,20  | 32,20  | OL     | OL     | OL     | OL     |
+--------+--------+--------+--------+--------+--------+--------+--------+
```

(OL = Overlay layer, seconde couche optionnelle)

Chaque partie du corps a 6 faces (top, bottom, right, front, left, back)
qui correspondent a des zones specifiques de la texture.

#### Rendu isometrique 2D (fallback / previews)

Pour les previews rapides et les listes, un rendu 2D isometrique :
- Vue 3/4 face (comme NameMC dans la galerie)
- Rendu cote serveur en Rust (crate `mc-skin`) avec la lib `image`
- Utilise pour OpenGraph, thumbnails, galerie rapide

---

## Animations & Effets

### Particules Hero
- Blocs MC pixelises (dirt, stone, diamond) flottant lentement
- Effet parallaxe au scroll
- Implementation : Canvas 2D ou CSS animations

### Transitions de page
- Fade + slide pour les changements de page (SvelteKit transitions)
- Skeleton loaders pendant le chargement des donnees

### Scroll animations
- Reveal au scroll (fade-in + translate-up) pour les sections
- Counter animation pour les chiffres (nombre de joueurs, etc.)

### Hover effects
- Cards : leser translateY(-4px) + ombre amplifiee
- Boutons : glow effect + scale(1.02)
- Liens joueurs : underline animate + couleur transition

### Skin viewer
- Rotation auto lente quand pas d'interaction
- Animation idle : leger balancement des bras, respiration (scale Y du body)
- Transition smooth quand on charge un nouveau skin

---

## Responsive Design

### Breakpoints
```css
/* Mobile first */
sm: 640px    /* Telephone large */
md: 768px    /* Tablette */
lg: 1024px   /* Desktop petit */
xl: 1280px   /* Desktop */
2xl: 1536px  /* Grand ecran */
```

### Adaptations mobiles
- Search bar : pleine largeur, padding reduit
- Server card : stack vertical (favicon au-dessus, infos en-dessous)
- Skin viewer : taille reduite, controles touch (pinch zoom, swipe rotate)
- Player profile : skin viewer en haut, infos en-dessous
- Navigation : burger menu

---

## SEO & Meta Tags

Chaque page doit avoir :

```html
<title>play.hypixel.net - Minecraft Server Status | MCInfo</title>
<meta name="description" content="Check Hypixel server status: 42,389 players online...">

<!-- OpenGraph -->
<meta property="og:title" content="Hypixel - Server Status">
<meta property="og:description" content="42,389 / 200,000 players online">
<meta property="og:image" content="https://mcinfo.rs/api/v1/render/og/server/play.hypixel.net">
<meta property="og:type" content="website">

<!-- Twitter Card -->
<meta name="twitter:card" content="summary_large_image">
```

Pour les pages joueurs, l'OG image = rendu du skin du joueur.
Pour les pages serveurs, l'OG image = card avec favicon + MOTD + stats.

---

## Performance

- **SSR** : Les pages server et player sont rendues cote serveur (SvelteKit)
  pour le SEO et le temps de chargement initial
- **Lazy loading** : Le viewer 3D Three.js est charge en lazy (pas dans le bundle initial)
- **Image optimization** : Favicons et skins servis en WebP quand supporte
- **Cache client** : SWR pattern (stale-while-revalidate) pour les donnees API
- **Code splitting** : Chaque page = un chunk separe
