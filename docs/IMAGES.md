# MCInfo-RS — Prompts de génération d'images

Ce fichier centralise les prompts pour générer les assets visuels du site via un outil de génération d'images (Midjourney, DALL-E 3, Stable Diffusion, Flux, etc.).

Toutes les images doivent être **au format 16:9** (recommandé : 1920×1080 ou 2560×1440) sauf indication contraire.

---

## Hero principal

### `/images/hero/hero-main-v01.png`

**Usage :** Hero de la page d'accueil, fond animé Ken Burns `zoom-in`.

```
Cinematic wide-angle screenshot of a Minecraft world at golden hour,
photorealistic shader pack (BSL or Complementary), lush biome with
rolling hills, ancient oak trees and a distant mountain range.
A vast sky with volumetric clouds catching the warm amber light.
Foreground slightly dark to accommodate overlaid text.
16:9, ultra-wide, high detail, no HUD, no player model.
Colors: deep blue-gray sky (#b4cad9 tones), golden sun, rich greens.
Mood: epic, inviting, discovery.
```

---

## Pages d'erreur

> Les images d'erreur servent de fond plein-écran avec un overlay gradient sombre.
> Elles doivent être **sombres et atmosphériques** pour que le texte blanc reste lisible.
> Format recommandé : **1920×1080**, orientation paysage.
> Zone de droite légèrement plus claire (la zone de texte est centrée).

---

### `/images/errors/error-404-v01.png`

**Usage :** Page d'erreur 404 — "Chunk introuvable".

**Concept :** Un joueur Minecraft solitaire debout au bord du vide absolu — le dernier chunk généré, au-delà duquel il n'y a que le néant. Sentiment de perte et de solitude.

```
Cinematic Minecraft screenshot with photorealistic shaders (BSL or Complementary),
a lone Steve player character standing at the very edge of the last generated chunk,
looking out into an infinite void of deep darkness and emptiness.
The terrain abruptly ends in a sharp cliff face revealing the absolute void below and beyond.
Dramatic side lighting, the last sunlight catching the edge of the terrain.
Deep dark blues and blacks in the void (#050608, #0b131c tones),
muted stone grey cliff face, sparse dead grass at the edge.
Volumetric mist rising from the void below.
Camera angle: slightly elevated, wide shot, player seen from behind.
Mood: mysterious, lost, desolate, cinematic.
16:9, no HUD, high detail.
```

---

### `/images/errors/error-5xx-v01.png`

**Usage :** Pages d'erreur serveur (500, 502, 503…) — "Le serveur a crashé".

**Concept :** Un serveur Minecraft qui explose — TNT en chaîne, feu et débris blocky dans un datacenter pixelisé ou une salle de serveurs en ruines, ambiance post-apocalyptique.

```
Cinematic Minecraft screenshot with photorealistic shaders,
massive TNT chain explosion destroying a server room built in Minecraft blocks,
multiple TNT blocks detonating simultaneously with bright orange-white blast cores,
debris of redstone, iron blocks and circuits flying outward.
Dark and dramatic atmosphere, thick smoke plumes rising (#050608 background),
ember particles glowing amber-orange (#ffc33d, #eb9d2a).
Flames casting dramatic volumetric light onto surrounding debris.
Camera angle: dramatic low-angle wide shot, explosion center slightly off-center.
Mood: chaotic, dramatic, high-energy, cinematic catastrophe.
16:9, no HUD, high detail, motion blur on debris.
```

---

### `/images/errors/error-generic-v01.png`

**Usage :** Toutes autres erreurs (400, 401, 403…) — accès refusé ou requête invalide.

**Concept :** Une porte en obsidienne ou un portail du Nether éteint, profondément underground, avec des inscriptions mystérieuses. Sentiment d'accès bloqué, de zone interdite.

```
Cinematic Minecraft screenshot with photorealistic shaders,
a massive ancient obsidian gate or portal frame deep underground,
unlit Nether portal (dark purple-black obsidian frame, void inside).
Surrounding walls of carved stone bricks with mysterious glowing runes.
Single torch flickering on the wall to the left, casting warm amber shadows.
Lava glow from a crack in the distant floor providing red-orange backlighting.
Deep dark atmosphere, mostly blacks and dark blues (#050608, #0b131c),
obsidian surfaces reflecting the torch light.
Camera: low-angle centered, portal filling 60% of the frame.
Mood: forbidden, mysterious, ancient, imposing.
16:9, no HUD, high detail.
```

---

## Notes techniques

- **Taille fichier cible :** < 800 Ko par image (compresser en WebP si possible)
- **Format sortie :** PNG (source) + WebP (production)
- **Placement :** Toutes les images dans `apps/web/static/images/`
- **Overlay :** Les images d'erreur sont toujours affichées avec un gradient overlay sombre appliqué en CSS — ne pas pré-assombrir les images manuellement
- **Cohérence :** Conserver le même pack de shaders / style visuel sur toutes les images du site
