<div align="center">

<img src="apps/web/static/images/logo/logo-mark-v01.png" alt="MCInfo-RS" width="120" />

# MCInfo-RS

**Minecraft Server & Player Intelligence Platform**

A self-contained Rust backend + SvelteKit frontend for querying Minecraft servers directly via the MC Protocol, looking up players through the Mojang API, and rendering skins in 2D/3D.

*We are the tier service, not the consumer.*

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte_5-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)
![Three.js](https://img.shields.io/badge/Three.js-000000?style=for-the-badge&logo=threedotjs&logoColor=white)
![wgpu](https://img.shields.io/badge/wgpu-4285F4?style=for-the-badge&logo=webgpu&logoColor=white)

</div>

<br/>

<div align="center">
  <img src="apps/web/static/images/hero/hero-main-v02.png" alt="MCInfo-RS Hero" width="100%" style="border-radius: 12px;" />
</div>

<br/>

## Features

- **Server Lookup** — Ping any Java/Bedrock server in real-time (SLP, Query, Raknet)
- **DNS SRV Resolution** — Automatic `_minecraft._tcp` SRV record lookup
- **MOTD Rendering** — Parse `§` color codes & Chat Components, render to HTML
- **Player Profiles** — UUID, skin, cape, name history via Mojang API
- **2D Skin Rendering** — Server-side isometric renders (head, bust, full body)
- **3D Skin Rendering** — Server-side wgpu renders + client-side Three.js interactive viewer
- **Smart Caching** — In-memory TTL cache (moka) for all API responses
- **Rate Limiting** — Per-endpoint rate limits with standard headers
- **Gaming UI** — Dark, immersive frontend inspired by NameMC & OriginRealms

---

## Architecture

```
┌─────────────────────────────────────┐
│         SvelteKit Frontend          │
│   (Three.js 3D Viewer, Gaming UI)  │
└──────────────┬──────────────────────┘
               │ REST API
┌──────────────▼──────────────────────┐
│          mc-api (Axum)              │
└──┬───────┬────────┬─────────┬───────┘
   │       │        │         │
   ▼       ▼        ▼         ▼
mc-protocol  mc-mojang  mc-skin   mc-render3d
   │                    mc-motd   mc-cache
   ▼
Minecraft Servers
 (direct ping)
```

### Crates

| Crate | Description |
|-------|-------------|
| **mc-protocol** | Minecraft protocol implementation (SLP, Query, Bedrock Raknet, DNS SRV) |
| **mc-api** | REST API server (Axum) — routes, middleware, rate limiting |
| **mc-mojang** | Mojang API client (UUID resolution, skin/cape fetching) |
| **mc-motd** | MOTD parser & renderer (`§` codes + Chat Component JSON → HTML/plain) |
| **mc-skin** | Skin downloading, model detection (slim/classic), 2D isometric rendering |
| **mc-render3d** | Server-side 3D skin rendering via wgpu (PNG output) |
| **mc-cache** | Generic async TTL cache backed by moka |

---

## API Endpoints

Base URL: `http://localhost:3001/api/v1`

### Servers

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/server/{address}` | Ping a Minecraft server (Java/Bedrock/auto) |
| `GET` | `/server/{address}/history` | Server uptime & player history |

### Players

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/player/{username}` | Player profile (UUID, skin, cape, name history) |
| `GET` | `/player/{username}/skin` | Raw skin PNG file |

### Skin Rendering

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/render/{username}` | 2D skin render (head/bust/full/face, 8-512px) |

### Utilities

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/stats` | Service statistics |

<details>
<summary><strong>Example response — Server Lookup</strong></summary>

```json
{
  "online": true,
  "address": {
    "hostname": "play.hypixel.net",
    "ip": "172.65.128.35",
    "port": 25565,
    "srv_record": true
  },
  "version": {
    "name": "Requires MC 1.8 / 1.21",
    "protocol": 47
  },
  "players": {
    "online": 42389,
    "max": 200000,
    "sample": [
      { "name": "Technoblade", "uuid": "b876ec32-e396-476b-a115-8438d83c67d4" }
    ]
  },
  "motd": {
    "raw": "§6§lHypixel Network §e[1.8-1.21]",
    "clean": "Hypixel Network [1.8-1.21]",
    "html": "<span style=\"color:#FFAA00;font-weight:bold\">Hypixel Network</span> <span style=\"color:#FFFF55\">[1.8-1.21]</span>"
  },
  "favicon": "data:image/png;base64,...",
  "latency_ms": 34,
  "edition": "java"
}
```

</details>

<details>
<summary><strong>Example response — Player Lookup</strong></summary>

```json
{
  "uuid": "b876ec32-e396-476b-a115-8438d83c67d4",
  "username": "Technoblade",
  "skin": {
    "url": "http://textures.minecraft.net/texture/abc123...",
    "model": "classic",
    "slim": false
  },
  "cape": {
    "url": "http://textures.minecraft.net/texture/def456..."
  },
  "name_history": [
    { "name": "Technoblade", "changed_at": null }
  ]
}
```

</details>

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Bun](https://bun.sh/) (for the frontend)

### Backend

```bash
# Run the API server (port 3001)
cargo run -p mc-api
```

### Frontend

```bash
cd apps/web
bun install
bun run dev
```

The frontend connects to `http://127.0.0.1:3001` by default. Override with a `.env.local`:

```env
PUBLIC_API_BASE=http://custom-url:3001
```

---

## Rate Limits

| Endpoint | Limit | Window |
|----------|-------|--------|
| `/server/*` | 30 req | 1 min |
| `/player/*` | 60 req | 1 min |
| `/render/*` | 60 req | 1 min |

---

## Cache Strategy

| Data | TTL |
|------|-----|
| Server ping | 60s |
| Player profile | 5 min |
| Skin PNG | 1h |
| Skin render | 1h |
| DNS SRV | 5 min |

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| API Framework | Axum 0.8 |
| Async Runtime | Tokio |
| MC Protocol | Custom implementation (zero third-party) |
| Mojang API | reqwest |
| Cache | moka (async TTL) |
| 3D Rendering (server) | wgpu + glam |
| Image Processing | image (PNG, WebP) |
| Frontend | SvelteKit 2 + Svelte 5 |
| 3D Rendering (client) | Three.js |
| Package Manager | Bun |

---

## Project Structure

```
.
├── crates/
│   ├── mc-protocol/     # Minecraft protocol (SLP, Query, Bedrock, DNS)
│   ├── mc-api/          # REST API server (Axum)
│   ├── mc-mojang/       # Mojang API client
│   ├── mc-motd/         # MOTD parser & HTML renderer
│   ├── mc-skin/         # Skin fetching & 2D rendering
│   ├── mc-render3d/     # Server-side 3D rendering (wgpu)
│   └── mc-cache/        # Generic TTL cache
├── apps/
│   └── web/             # SvelteKit frontend
├── docs/                # Architecture & API documentation
└── Cargo.toml           # Workspace root
```

---

## License

This project is proprietary. All rights reserved.
