# MCInfo-RS - Documentation API REST

API REST publique exposee par la crate `mc-api` (Axum).
Toutes les donnees sont obtenues **directement** depuis les serveurs MC
et l'API Mojang officielle. Aucun service tier.

Base URL : `https://api.mcinfo.rs/v1` (a definir)

---

## Endpoints

### Serveurs

#### `GET /api/v1/server/{address}`

Ping un serveur Minecraft et retourne ses informations en temps reel.

**Parametres path :**
- `address` : IP ou domaine du serveur. Formats acceptes :
  - `play.hypixel.net`
  - `play.hypixel.net:25565`
  - `172.65.128.35`
  - `172.65.128.35:25565`

**Parametres query :**
- `type` (optionnel) : `java` | `bedrock` | `auto` (defaut: `auto`)
- `query` (optionnel) : `true` | `false` - tenter le Query Protocol UDP (defaut: `false`)

**Reponse 200 :**
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
      {
        "name": "Technoblade",
        "uuid": "b876ec32-e396-476b-a115-8438d83c67d4"
      }
    ]
  },
  "motd": {
    "raw": "\u00a76\u00a7lHypixel Network \u00a7e[1.8-1.21]",
    "clean": "Hypixel Network [1.8-1.21]",
    "html": "<span style=\"color:#FFAA00;font-weight:bold\">Hypixel Network</span> <span style=\"color:#FFFF55\">[1.8-1.21]</span>"
  },
  "favicon": "data:image/png;base64,...",
  "latency_ms": 34,
  "edition": "java",
  "retrieved_at": "2026-03-10T14:30:00Z"
}
```

**Reponse 200 (serveur offline) :**
```json
{
  "online": false,
  "address": {
    "hostname": "play.example.com",
    "ip": "1.2.3.4",
    "port": 25565,
    "srv_record": false
  },
  "error": "connection_refused",
  "retrieved_at": "2026-03-10T14:30:00Z"
}
```

**Erreurs :**
- `400` : Adresse invalide
- `422` : DNS resolution failed
- `429` : Rate limit exceeded
- `504` : Timeout (serveur ne repond pas)

---

#### `GET /api/v1/server/{address}/history`

Historique du serveur (uptime, joueurs dans le temps).

**Parametres query :**
- `period` : `1h` | `6h` | `24h` | `7d` | `30d` (defaut: `24h`)

**Reponse 200 :**
```json
{
  "address": "play.hypixel.net",
  "period": "24h",
  "data_points": [
    {
      "timestamp": "2026-03-09T14:00:00Z",
      "online": true,
      "players_online": 41230,
      "latency_ms": 32
    },
    {
      "timestamp": "2026-03-09T14:15:00Z",
      "online": true,
      "players_online": 41580,
      "latency_ms": 35
    }
  ],
  "summary": {
    "uptime_percent": 99.8,
    "avg_players": 40500,
    "peak_players": 52000,
    "min_players": 28000,
    "avg_latency_ms": 33
  }
}
```

---

### Joueurs

#### `GET /api/v1/player/{username}`

Informations sur un joueur Minecraft via l'API Mojang officielle.

**Parametres path :**
- `username` : Pseudo Minecraft (3-16 caracteres) ou UUID

**Reponse 200 :**
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
    {
      "name": "Technoblade",
      "changed_at": null
    }
  ],
  "skin_history": [
    {
      "url": "http://textures.minecraft.net/texture/abc123...",
      "first_seen": "2026-01-15T00:00:00Z",
      "model": "classic"
    }
  ],
  "retrieved_at": "2026-03-10T14:30:00Z"
}
```

**Erreurs :**
- `400` : Username invalide (format)
- `404` : Joueur non trouve
- `429` : Rate limit exceeded

---

#### `GET /api/v1/player/{username}/skin`

Retourne directement le fichier PNG du skin.

**Parametres query :**
- `download` : `true` (ajoute Content-Disposition) | `false` (defaut)

**Reponse 200 :**
- `Content-Type: image/png`
- Body : fichier PNG 64x64 (ou 64x32 legacy)

---

### Rendu Skins

#### `GET /api/v1/render/{username}`

Rendu 2D du skin cote serveur (pour previews, embeds, OpenGraph).

**Parametres query :**
- `type` : `head` | `bust` | `full` | `face` (defaut: `full`)
- `size` : 8-512 pixels (defaut: 128)
- `overlay` : `true` | `false` - inclure la layer overlay (defaut: `true`)
- `format` : `png` | `webp` (defaut: `png`)

**Reponse 200 :**
- `Content-Type: image/png` (ou `image/webp`)
- Body : image rendue

**Exemples d'URL :**
```
/api/v1/render/Dream?type=head&size=64
/api/v1/render/Dream?type=full&size=256&overlay=true
/api/v1/render/b876ec32-e396-476b-a115-8438d83c67d4?type=bust
```

---

### Utilitaires

#### `GET /health`

Health check du service.

```json
{
  "status": "ok",
  "version": "0.1.0",
  "uptime_seconds": 86400
}
```

#### `GET /api/v1/stats`

Statistiques du service.

```json
{
  "servers_queried_total": 125000,
  "players_looked_up_total": 89000,
  "cache_hit_rate": 0.72,
  "active_tracking": 500
}
```

---

## Rate Limiting

| Endpoint | Limite | Fenetre |
|----------|--------|---------|
| `/api/v1/server/*` | 30 requetes | par minute |
| `/api/v1/player/*` | 60 requetes | par minute |
| `/api/v1/render/*` | 60 requetes | par minute |

Headers de reponse :
```
X-RateLimit-Limit: 30
X-RateLimit-Remaining: 28
X-RateLimit-Reset: 1710085860
```

Reponse 429 :
```json
{
  "error": "rate_limit_exceeded",
  "retry_after_seconds": 42
}
```

---

## Cache

| Donnee | TTL | Justification |
|--------|-----|---------------|
| Server ping | 60s | Les infos changent souvent |
| Player profile | 5min | Les skins changent rarement |
| Skin PNG | 1h | Le fichier image change tres rarement |
| Skin render | 1h | Depend du skin, meme TTL |
| DNS SRV | 5min | Les records DNS changent rarement |

---

## Format d'erreurs

Toutes les erreurs suivent le meme format :

```json
{
  "error": "error_code",
  "message": "Human-readable description",
  "details": {}
}
```

Codes d'erreur :
- `invalid_address` : Adresse serveur invalide
- `dns_resolution_failed` : Impossible de resoudre le domaine
- `connection_refused` : Serveur ne repond pas
- `timeout` : Timeout de connexion
- `player_not_found` : Joueur inexistant
- `rate_limit_exceeded` : Trop de requetes
- `internal_error` : Erreur interne
