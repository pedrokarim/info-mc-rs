# MCInfo Web (SvelteKit + Bun)

Frontend officiel de `info-mc-rs`.

## Stack

- SvelteKit 2 + Svelte 5
- TypeScript strict
- Bun (`bun install`, `bun run ...`)

## DA (direction artistique)

DA originale MCInfo inspirée des codes de qualité observés sur les sites gaming:

- Fond sombre immersif avec contrastes forts
- CTA or marqués pour les actions principales
- Cartes data lisibles (server/player/skin)
- Composants unifiés: boutons, chips, surfaces, badges de statut
- Hiérarchie claire entre sections éditoriales et sections lookup

## Routes

- `/` home + lookup global (server / player)
- `/server/[address]` fiche serveur + MOTD visuel + stats joueurs
- `/skins` recherche skin + galerie type NameMC
- `/player/[username]` détail skin type NameMC

## Configuration API

Créer un `.env` (ou `.env.local`) dans `apps/web`:

```bash
PUBLIC_API_BASE=http://127.0.0.1:3002
```

Si absent, fallback automatique sur `http://127.0.0.1:3002`.

## Commandes Bun

```bash
bun install
bun run dev
bun run check
bun run build
```
