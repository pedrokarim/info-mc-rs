# NBT Viewer — Méthode de mapping textures blocs

## Problème

Les fichiers `.nbt` (structures Minecraft) contiennent des noms de blocs qui ne correspondent pas toujours directement aux fichiers de textures 16×16 extraits du client JAR.

**3 types de problèmes :**

1. **Nom différent** : `snow_block` → `snow.png`, `grass_block` → `grass_block_top.png`
2. **Blocs variants** (pre-1.13) : `stone` + `{variant: "granite"}` → `granite.png`
3. **Blocs dérivés** : `dark_oak_stairs` n'a pas de texture, il faut utiliser `dark_oak_planks.png`

## Architecture de résolution

### 1. Overrides explicites (`block-textures.ts`)

```ts
const TEXTURE_OVERRIDES: Record<string, string> = {
  'grass_block': 'grass_block_top',
  'snow_block': 'snow',
  'planks': 'oak_planks',        // legacy
  'web': 'cobweb',               // legacy
  // ...
};
```

### 2. Fallback automatique par suffixe

Si le nom contient `_slab`, `_stairs`, `_wall`, `_fence`, etc., on strip le suffixe et on utilise la texture du bloc de base :

```
dark_oak_stairs → strip _stairs → dark_oak → wood type → dark_oak_planks.png
cobblestone_wall → strip _wall → cobblestone.png
stone_brick_slab → strip _slab → stone_bricks (via override) → stone_bricks.png
```

**Wood types** reconnus : oak, spruce, birch, jungle, acacia, dark_oak, cherry, mangrove, bamboo, crimson, warped.

### 3. Résolution des properties NBT (`resolveBlockName()`)

Pour les vieilles structures (pre-1.13 "The Flattening"), les blocs utilisent des properties `variant` :

```
minecraft:stone + {variant: "granite"} → minecraft:granite
minecraft:stonebrick + {variant: "mossy"} → minecraft:mossy_stone_bricks
minecraft:coral_block + {coral_color: "tube"} → minecraft:tube_coral_block
```

### 4. Texture missing (fallback ultime)

Si aucune texture n'est trouvée, on affiche le **damier noir/magenta** classique de Minecraft (généré par canvas 16×16).

Le matériau Three.js est créé avec la texture missing par défaut. Si le chargement réussit, la `.map` est remplacée par la vraie texture. Si le 404, le damier reste.

## Comment ajouter un nouveau mapping

1. Charger une structure qui a des blocs manquants (damier rose/noir)
2. Identifier les noms de blocs dans l'arbre NBT (panneau gauche)
3. Vérifier si le fichier texture existe :
   ```bash
   ls static/images/blocks/{nom_du_bloc}.png
   ```
4. Si le fichier a un nom différent, ajouter dans `TEXTURE_OVERRIDES` :
   ```ts
   'nom_nbt': 'nom_fichier_texture',
   ```
5. Si c'est un bloc legacy (pre-1.13), vérifier le mapping `variant` dans `resolveBlockName()`

## Script de vérification

Pour scanner tous les NBT et trouver les blocs sans texture :

```bash
python3 << 'EOF'
import gzip, re, os, glob

BLOCKS = 'apps/web/static/images/blocks'
STRUCTS = 'apps/web/static/structures'

all_blocks = set()
for f in glob.glob(f'{STRUCTS}/**/*.nbt', recursive=True):
    data = open(f, 'rb').read()
    if data[0:2] == b'\x1f\x8b': data = gzip.decompress(data)
    all_blocks.update(n.decode() for n in re.findall(b'minecraft:[a-z_]+', data))

all_blocks -= {'minecraft:air', 'minecraft:cave_air', 'minecraft:void_air', 'minecraft:structure_void', 'minecraft:structure_block'}

for b in sorted(all_blocks):
    short = b.replace('minecraft:', '')
    if not os.path.exists(f'{BLOCKS}/{short}.png'):
        print(f'MISSING: {short}')
EOF
```

## Extraction des textures

Les textures proviennent du client Minecraft JAR officiel :

```bash
# Télécharger le JAR
curl -sL "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json" | \
  python3 -c "import sys,json; d=json.load(sys.stdin); v=[x for x in d['versions'] if x['id']==d['latest']['release']][0]; print(v['url'])" | \
  xargs curl -sL | python3 -c "import sys,json; print(json.load(sys.stdin)['downloads']['client']['url'])" | \
  xargs curl -sLo client.jar

# Extraire les textures bloc
unzip -qo client.jar "assets/minecraft/textures/block/*"
cp assets/minecraft/textures/block/*.png apps/web/static/images/blocks/
```
