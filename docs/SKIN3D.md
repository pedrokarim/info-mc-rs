# 3D Skin Viewer — Notes techniques

Reference: `.reference/script.js` (code Three.js extrait de NameMC)

## Architecture du modèle 3D

Le skin MC est composé de **6 groupes** (Object3D) positionnés ainsi :

| Partie      | Position (x, y, z)       | BoxGeometry (w, h, d)     | Segments           |
|-------------|---------------------------|----------------------------|--------------------|
| Head        | (0, 12, 0)               | 8 × 8 × 8                 | 8 × 8 × 8         |
| Torso       | (0, 2, 0)                | 8 × 12 × 4                | 8 × 12 × 4        |
| Right Arm   | (-6 / -5.5 slim, 6, 0)  | 4 × 12 × 4 (3 si slim)    | w × 12 × 4        |
| Left Arm    | (6 / 5.5 slim, 6, 0)    | 4 × 12 × 4 (3 si slim)    | w × 12 × 4        |
| Right Leg   | (-2, -4, 0)              | 4 × 12 × 4                | 4 × 12 × 4        |
| Left Leg    | (2, -4, 0)               | 4 × 12 × 4                | 4 × 12 × 4        |
| Cape        | (0, 8, -2) + rot Y 180°  | 10 × 16 × 1               | 10cs × 16cs × cs   |

Les bras et jambes ont un `translate(0, -4, 0)` ou `translate(0, -6, 0)` pour décaler le pivot de rotation vers le haut de la pièce.

## Overlay layers (64x64 skins uniquement)

Chaque partie a une couche overlay légèrement plus grande (+ 0.5px + epsilon) :

| Overlay       | Taille delta  | Epsilon multiplier |
|---------------|---------------|--------------------|
| Hat           | +1 px         | —                  |
| Jacket        | +0.5 + eps    | ×1                 |
| Right Sleeve  | +0.5 + eps×4  | ×4                 |
| Left Sleeve   | +0.5 + eps×4  | ×4                 |
| Right Pant    | +0.5 + eps×2  | ×2                 |
| Left Pant     | +0.5 + eps×3  | ×3                 |

Les epsilon différents évitent le z-fighting entre les couches.

## UV Mapping — Tableau SKIN

Le tableau `SKIN[v][part][layer]` contient les rectangles UV par face de cube.
- `v = 0` : format legacy 64×32 (pas d'overlays sauf hat)
- `v = 1` : format moderne 64×64 (overlays complètes)

Chaque face de box = `[x, y, w, h]` dans la texture skin.
6 faces par box dans l'ordre: **right, left, top, bottom, front, back**

Signe négatif sur `w` ou `h` = flip horizontal/vertical.

### Coordonnées UV Head (v=1)
```
Head:   [[16,8,8,8], [0,8,8,8], [8,0,8,8], [16,7,8,-8], [8,8,8,8], [24,8,8,8]]
Hat:    [[48,8,8,8], [32,8,8,8], [40,0,8,8], [48,7,8,-8], [40,8,8,8], [56,8,8,8]]
```

### Slim vs Classic
Les bras slim utilisent un index dans le tableau: `SKIN[v][2][layer][slim]`
- `slim = 0` → largeur 4px (classic)
- `slim = 1` → largeur 3px (slim/Alex)

## Animation

Balancement sinusoïdal avec `sin(radians(time))` :

```
rightArm.rotation.x = -18° × sin(t)
leftArm.rotation.x  = +18° × sin(t)
rightLeg.rotation.x = +20° × sin(t)
leftLeg.rotation.x  = -20° × sin(t)
cape.rotation.x     = 18° - 6° × sin(t/4)
```

Cycle complet = 1440 unités de temps, vitesse = 360/1500ms.

## Rotation interactive (drag)

- `theta` : rotation horizontale (Y-axis), illimitée
- `phi` : rotation verticale (X-axis), clampée à [-90°, +90°]
- Position initiale : theta=30°, phi=21°
- Support mouse + touch (multi-touch avec identifiers)

## Eclairage

```
AmbientLight(0xFFFFFF, 0.7)
DirectionalLight(0xFFFFFF, 0.3) → position(0.678, 0.284, 0.678)
```

## Camera

```
PerspectiveCamera(fov=38, near=40, far=80)
position.z = 60
lookAt(0, 0, 0)
```

## Technique de coloration (colorFaces)

NameMC n'utilise PAS de textures UV classiques. A la place :
1. Lecture pixel par pixel du bitmap skin
2. Chaque pixel = 2 triangles (face) avec `face.color` RGB
3. Les pixels avec alpha < 255 → `MeshLambertMaterial` transparent
4. Les pixels avec alpha = 0 → faces supprimées
5. Materials groupés par valeur d'alpha (optimisation)
6. `DoubleSide` activé si le layer contient de la transparence

## Cape scaling

Détection du format de cape par hauteur :
- `height % 22 === 0` → scale = height/22 (format standard)
- `height % 17 === 0` → scale = height/17 (format Optifine)
- Power of 2 && >= 32 → scale = height/32 (format HD)
- Fallback → max(1, floor(height/22))

## Détection legacy vs moderne

```
v = (skin.height >= 64) ? 1 : 0
```

Si `v = 0` (64×32), pas d'overlays sauf le hat. Si hasAlpha est false, on utilise les pixels opaques directement.

## Implementation plan pour notre projet

Pour le frontend, on utilisera **Three.js** (ou son wrapper framework si SvelteKit/React) avec la même approche :
1. Parser le skin PNG en pixels côté client (Canvas 2D → getImageData)
2. Construire le modèle 3D avec BoxGeometry per-pixel coloring OU texture UV mapping classique
3. Animation loop avec requestAnimationFrame
4. Drag rotation avec mouse/touch events
5. Support slim/classic via le `model` field retourné par notre API `/api/v1/player/{id}`
