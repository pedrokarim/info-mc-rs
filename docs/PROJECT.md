# MCInfo-RS - Minecraft Server & Player Intelligence Platform

## Vision

Service tier autonome en Rust pour interroger directement les serveurs Minecraft
(sans passer par des API tierces) et offrir une interface web gaming-style pour
visualiser les informations serveurs, joueurs, skins et rendus 3D.

**On est le service tier, pas le consommateur.**

---

## Stack Technique

| Couche | Technologie | Justification |
|--------|------------|---------------|
| Backend / API | **Rust** (Axum) | Performance, safety, async natif |
| Protocole MC | Implem custom du **MC Protocol** (SLP / Query) | Zero dependance tierce |
| Base de donnees | **SQLite** (via sqlx) ou **PostgreSQL** | Historique serveurs, cache skins |
| Frontend | **SvelteKit** ou **Next.js** (a definir) | UI reactive, SSR pour SEO |
| Rendu 3D Skins | **Three.js** / **WebGL** cote client | Rendu temps reel des skins MC |
| Cache | **Redis** ou in-memory (moka) | Rate limiting, cache reponses |

---

## Architecture Globale

```
                    +-------------------+
                    |   Frontend Web    |
                    |  (Gaming UI/UX)   |
                    +--------+----------+
                             |
                         REST / WebSocket
                             |
                    +--------v----------+
                    |    API Rust       |
                    |    (Axum)         |
                    +--------+----------+
                             |
              +--------------+--------------+
              |              |              |
     +--------v---+  +------v------+  +----v--------+
     | MC Protocol |  | Mojang API  |  |  Cache /    |
     | Client      |  | (Skins/UUID)|  |  Database   |
     | (SLP/Query) |  |             |  |             |
     +-------------+  +-------------+  +-------------+
           |
     +-----v-----------+
     | Serveurs MC      |
     | (ping direct)    |
     +------------------+
```

---

## Fonctionnalites

### 1. Server Lookup (coeur du projet)

L'utilisateur entre une IP (+ port optionnel) et obtient :

#### Donnees recuperees via le protocole MC directement

- **Server List Ping (SLP)** - Protocole TCP standard MC :
  - Version du serveur (nom + numero de protocole)
  - MOTD (Message of the Day) avec formatage couleurs/styles MC
  - Nombre de joueurs (online / max)
  - Liste partielle des joueurs connectes (sample)
  - Favicon du serveur (base64 PNG 64x64)
  - Latence (temps de reponse du ping)

- **Query Protocol** (UDP, port 25565 par defaut) - si active sur le serveur :
  - Liste complete des plugins
  - Map actuelle
  - Type de jeu
  - Version logicielle
  - Liste complete des joueurs

- **Bedrock Edition** (UDP Raknet) :
  - Support du ping Bedrock via le protocole Unconnected Ping

#### Donnees enrichies

- Historique de disponibilite (uptime tracking)
- Historique du nombre de joueurs (graphiques)
- Resolution DNS SRV pour les domaines MC
- Detection du type de serveur (Vanilla, Spigot, Paper, Bungeecord, Velocity...)
- Geolocalisation IP (via base GeoIP locale, pas d'API tierce)

### 2. Player Lookup (style NameMC)

#### Via l'API Mojang officielle (seule dependance externe justifiee)

- **UUID Resolution** : Username -> UUID via `api.mojang.com`
- **Profil joueur** : Skin URL, cape, modele (slim/classic)
- **Historique des noms** (si toujours disponible)

#### Rendu 3D des Skins

- Telechargement de la texture skin depuis les CDN Mojang
- **Rendu WebGL/Three.js** dans le navigateur :
  - Vue complete du personnage (corps entier)
  - Vue buste / tete
  - Rotation interactive (drag)
  - Support des layers (overlay)
  - Support slim / classic arm models
  - Animation idle optionnelle
  - Export PNG du rendu (screenshot)

#### Features supplementaires skins

- Galerie de skins populaires / recentes
- Historique des skins d'un joueur (on stocke nous-memes au fil du temps)
- Telechargement du fichier skin (.png)
- Apercu flat (2D) + rendu 3D

### 3. API Publique

Exposer notre propre API REST pour que d'autres devs puissent l'utiliser :

```
GET /api/v1/server/{address}          -> Infos serveur
GET /api/v1/server/{address}/history  -> Historique joueurs/uptime
GET /api/v1/player/{username}         -> Profil joueur + skin
GET /api/v1/player/{username}/skin    -> Skin PNG brut
GET /api/v1/render/{username}         -> Rendu skin (head/body, taille, format)
```

---

## Implementation du Protocole MC (le coeur Rust)

### Server List Ping (SLP) - Java Edition

Flux TCP :

```
1. Connexion TCP a l'adresse:port (defaut 25565)
2. Envoyer Handshake Packet (ID 0x00)
   - Protocol Version (VarInt)
   - Server Address (String)
   - Server Port (Unsigned Short)
   - Next State = 1 (Status)
3. Envoyer Status Request (ID 0x00, vide)
4. Recevoir Status Response (ID 0x00)
   - JSON avec: version, players, description, favicon
5. Envoyer Ping (ID 0x01 + payload long)
6. Recevoir Pong (ID 0x01 + meme payload)
   - Calculer latence
7. Fermer connexion
```

### Types MC a implementer

- **VarInt** / **VarLong** : Encodage variable-length
- **MC String** : Prefixee par VarInt length, UTF-8
- **MC Packet** : Length (VarInt) + Packet ID (VarInt) + Data

### Query Protocol (UDP)

```
1. Envoyer Handshake (magic + type=9 + session)
2. Recevoir Challenge Token
3. Envoyer Stat Request avec le challenge token
   - Basic stat : MOTD, gametype, map, numplayers, maxplayers, hostport, hostip
   - Full stat : + plugins, players list
4. Parser la reponse key-value
```

### Bedrock Ping (Raknet)

```
1. Envoyer Unconnected Ping (ID 0x01)
   - Time (long) + Magic + Client GUID (long)
2. Recevoir Unconnected Pong (ID 0x1C)
   - Parser le server ID string (semicolon-delimited)
   - Edition, MOTD, protocol, version, players, max, server ID, map, gamemode...
```

---

## Design Frontend - Aesthetic Gaming

### Inspirations

- **Origin Realms** (originrealms.com) : Dark theme immersif, particules,
  typographie bold, effets de glow, sections hero plein ecran
- **ChunkLock** (chunklock.com) : UI clean pour lookup serveur, cards avec
  stats, previews serveurs
- **NameMC** : Reference pour le player lookup, historique skins, rendu 3D

### Direction Artistique

#### Palette de couleurs
- **Background** : Dark (#0a0a0f) avec des nuances de bleu-noir (#0d1117)
- **Primary** : Vert emeraude MC (#00d68f) ou Bleu electrique (#4f46e5)
- **Accent** : Violet/magenta (#a855f7) pour les highlights
- **Text** : Blanc (#f0f0f0) + gris doux (#94a3b8) pour le secondaire
- **Cards** : Fond semi-transparent avec glassmorphism + border subtle glow

#### Typographie
- **Headings** : Font bold/black, gaming feel (Inter Black, ou custom)
- **Body** : Inter / Geist pour la lisibilite
- **Monospace** : JetBrains Mono pour les IP, UUIDs, donnees techniques
- **MOTD MC** : Rendu custom avec les codes couleur MC (&a, &b, &c...)

#### Elements UI
- **Hero section** : Animation de particules (blocs MC flottants) ou fond
  parallax avec un monde MC en arriere-plan
- **Search bar** : Grande, centree, avec glow au focus, placeholder anime
- **Server cards** : Glassmorphism, favicon serveur, barre de joueurs animee,
  badge version, MOTD rendu avec couleurs MC
- **Player cards** : Rendu 3D du skin en temps reel, infos a cote
- **Graphiques** : Charts custom (recharts/chart.js) pour l'historique joueurs
- **Animations** : Transitions Framer Motion / GSAP, hover effects, scroll reveal
- **Easter eggs** : Creeper face loader, sons MC optionnels sur interactions

#### Responsive
- Mobile-first, le lookup doit etre utilisable sur telephone
- Layout adaptatif : stack vertical mobile, grid desktop

---

## Structure du Projet - Cargo Workspace Multi-Crates

Le projet est organise en **workspace Cargo** avec des crates independantes
et reutilisables. Chaque crate a une responsabilite claire.

```
info-mc-rs/
|-- Cargo.toml                      # [workspace] - racine du workspace
|-- docs/
|   |-- PROJECT.md                  # Ce fichier
|   |-- PROTOCOL.md                 # Details protocole MC
|   |-- API.md                      # Documentation API
|   |-- FRONTEND.md                 # Specs frontend
|   |-- ref-*.png                   # Screenshots sites de reference
|
|-- crates/
|   |-- mc-protocol/                # Crate: implementation protocole MC
|   |   |-- Cargo.toml
|   |   |-- src/
|   |   |   |-- lib.rs
|   |   |   |-- types.rs            # VarInt, VarLong, MC String, Packet
|   |   |   |-- slp.rs              # Server List Ping (Java TCP)
|   |   |   |-- query.rs            # Query Protocol (UDP)
|   |   |   |-- bedrock.rs          # Bedrock Raknet Ping
|   |   |   |-- dns.rs              # Resolution SRV records
|   |   |   |-- error.rs            # Erreurs protocole
|   |   |-- tests/
|   |       |-- slp_test.rs
|   |       |-- query_test.rs
|   |       |-- bedrock_test.rs
|   |
|   |-- mc-mojang/                  # Crate: client API Mojang officielle
|   |   |-- Cargo.toml
|   |   |-- src/
|   |   |   |-- lib.rs
|   |   |   |-- client.rs           # Client HTTP api.mojang.com
|   |   |   |-- types.rs            # Profile, Skin, Cape, UUID
|   |   |   |-- error.rs
|   |   |-- tests/
|   |
|   |-- mc-motd/                    # Crate: parser & renderer MOTD MC
|   |   |-- Cargo.toml
|   |   |-- src/
|   |   |   |-- lib.rs
|   |   |   |-- parser.rs           # Parse les codes couleur MC (§a, §b...)
|   |   |   |-- types.rs            # Arbre MOTD (texte + styles)
|   |   |   |-- html.rs             # Rendu MOTD -> HTML
|   |   |   |-- plain.rs            # Rendu MOTD -> texte brut
|   |
|   |-- mc-skin/                    # Crate: manipulation skins MC
|   |   |-- Cargo.toml
|   |   |-- src/
|   |   |   |-- lib.rs
|   |   |   |-- fetch.rs            # Telechargement skin depuis CDN Mojang
|   |   |   |-- model.rs            # Detection slim/classic, extraction parties
|   |   |   |-- render2d.rs         # Rendu 2D flat (face, corps, isometrique)
|   |   |   |-- cache.rs            # Cache local des skins PNG
|   |
|   |-- mc-db/                      # Crate: persistence & historique
|   |   |-- Cargo.toml
|   |   |-- src/
|   |   |   |-- lib.rs
|   |   |   |-- models.rs           # Modeles: Server, Player, SkinHistory, Uptime
|   |   |   |-- repo.rs             # Trait Repository + implem SQLite
|   |   |   |-- migrations.rs       # Setup schema
|   |   |-- migrations/
|   |       |-- 001_init.sql
|   |
|   |-- mc-cache/                   # Crate: cache in-memory generique
|   |   |-- Cargo.toml
|   |   |-- src/
|   |       |-- lib.rs              # Wrapper moka avec TTL configurable
|   |
|   |-- mc-api/                     # Crate: serveur API REST (Axum)
|       |-- Cargo.toml
|       |-- src/
|           |-- lib.rs
|           |-- main.rs             # Point d'entree, setup Axum + routes
|           |-- config.rs           # Configuration (env, fichiers)
|           |-- routes/
|           |   |-- mod.rs
|           |   |-- server.rs       # GET /api/v1/server/{address}
|           |   |-- player.rs       # GET /api/v1/player/{username}
|           |   |-- render.rs       # GET /api/v1/render/{username}
|           |   |-- health.rs       # GET /health
|           |-- middleware/
|           |   |-- mod.rs
|           |   |-- rate_limit.rs   # Rate limiting par IP
|           |   |-- cors.rs         # CORS config
|           |-- services/
|           |   |-- mod.rs
|           |   |-- server_service.rs
|           |   |-- player_service.rs
|           |   |-- tracking_service.rs
|           |-- error.rs            # Error handling API (JSON errors)
|           |-- state.rs            # AppState (pool DB, caches, clients)
|
|-- frontend/                       # Projet frontend (SvelteKit ou Next.js)
|   |-- src/
|   |   |-- routes/                 # Pages
|   |   |-- lib/
|   |   |   |-- components/         # Composants UI
|   |   |   |-- three/              # Rendu 3D Three.js (skin viewer)
|   |   |   |-- motd/               # Rendu MOTD cote client
|   |   |   |-- api/                # Client API vers mc-api
|   |   |-- styles/                 # Styles globaux, theme gaming
|   |-- static/                     # Assets statiques
|   |-- package.json
```

### Graphe de dependances des crates

```
mc-api (binaire principal)
  |-- mc-protocol        (ping serveurs)
  |-- mc-mojang          (profils joueurs)
  |-- mc-motd            (parser MOTD)
  |-- mc-skin            (manipulation skins)
  |-- mc-db              (persistence)
  |-- mc-cache           (cache in-memory)

mc-skin
  |-- mc-mojang          (fetch skin URL depuis profil)

mc-protocol
  (aucune dep interne, standalone)

mc-mojang
  (aucune dep interne, standalone)

mc-motd
  (aucune dep interne, standalone)
```

### Avantages du workspace multi-crates

- **Separation des responsabilites** : chaque crate fait une chose bien
- **Reutilisabilite** : `mc-protocol` peut etre publie sur crates.io independamment
- **Compilation incrementale** : modifier `mc-api` ne recompile pas `mc-protocol`
- **Tests isoles** : chaque crate a ses propres tests
- **CLI possible** : on peut ajouter une crate `mc-cli` qui reutilise `mc-protocol`
  pour faire un outil en ligne de commande

---

## Plan de Developpement (Phases)

### Phase 1 - Fondations Protocole
- [ ] Setup projet Rust (Cargo, deps)
- [ ] Implementer les types MC (VarInt, String, Packet)
- [ ] Implementer le Server List Ping (SLP) Java Edition
- [ ] Parser la reponse JSON (version, motd, players, favicon)
- [ ] Mesure de latence
- [ ] Resolution DNS SRV
- [ ] Tests unitaires protocole

### Phase 2 - API REST
- [ ] Setup Axum avec routing
- [ ] Endpoint GET /api/v1/server/{address}
- [ ] Gestion des erreurs (timeout, connexion refusee, DNS fail)
- [ ] Rate limiting basique
- [ ] Cache in-memory (moka)
- [ ] CORS configuration

### Phase 3 - Protocoles supplementaires
- [ ] Query Protocol UDP (basic + full stat)
- [ ] Bedrock Raknet Ping
- [ ] Detection automatique Java vs Bedrock

### Phase 4 - Player Lookup & Skins
- [ ] Client Mojang API (UUID lookup, profil)
- [ ] Telechargement et cache des skins
- [ ] Endpoint player lookup
- [ ] Endpoint skin brut (proxy/cache du PNG)

### Phase 5 - Frontend MVP
- [ ] Setup projet frontend
- [ ] Page d'accueil avec search bar hero
- [ ] Page resultat serveur (infos, MOTD rendu, joueurs)
- [ ] Page joueur (infos, skin 2D)
- [ ] Rendu MOTD avec codes couleur MC
- [ ] Design dark gaming theme

### Phase 6 - Rendu 3D Skins
- [ ] Setup Three.js / WebGL renderer
- [ ] Modele 3D personnage MC (geometrie des cubes)
- [ ] Mapping texture skin sur le modele
- [ ] Support layers (overlay armor layer)
- [ ] Support slim arms
- [ ] Rotation interactive (OrbitControls)
- [ ] Animation idle
- [ ] Export screenshot PNG

### Phase 7 - Historique & Tracking
- [ ] Base de donnees (schema, migrations)
- [ ] Tracking periodique des serveurs populaires
- [ ] Historique nombre de joueurs (time series)
- [ ] Historique uptime
- [ ] Graphiques dans le frontend
- [ ] Historique skins joueurs

### Phase 8 - Polish & Production
- [ ] API publique documentee (OpenAPI/Swagger)
- [ ] Rate limiting avance (par IP, par clef API)
- [ ] SEO / meta tags / OpenGraph
- [ ] Performance frontend (lazy loading, code splitting)
- [ ] Monitoring / health checks
- [ ] Deployment (Docker, CI/CD)

---

## Protocole MC - References

- Wiki.vg (protocol spec) : https://wiki.vg/Protocol
- Server List Ping : https://wiki.vg/Server_List_Ping
- Query : https://wiki.vg/Query
- Raknet / Bedrock : https://wiki.vg/Raknet_Protocol
- Mojang API : https://wiki.vg/Mojang_API
- MC color codes : https://minecraft.fandom.com/wiki/Formatting_codes

---

## Dependances Rust Envisagees

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }       # Async runtime
axum = "0.7"                                          # Web framework
serde = { version = "1", features = ["derive"] }      # Serialization
serde_json = "1"                                      # JSON
reqwest = { version = "0.12", features = ["json"] }   # HTTP client (Mojang API)
sqlx = { version = "0.7", features = ["sqlite"] }     # Database
moka = { version = "0.12", features = ["future"] }    # In-memory cache
tower-http = { version = "0.5", features = ["cors"] } # CORS middleware
tracing = "0.1"                                       # Logging
tracing-subscriber = "0.3"                             # Log formatting
trust-dns-resolver = "0.23"                            # DNS SRV resolution
base64 = "0.22"                                        # Favicon decoding
image = "0.25"                                         # Image processing
thiserror = "1"                                        # Error types
uuid = { version = "1", features = ["v4", "serde"] }  # UUID handling
chrono = { version = "0.4", features = ["serde"] }    # Timestamps
```

---

## Notes

- L'API Mojang est la seule dependance externe acceptee car c'est l'API
  **officielle** de Mojang/Microsoft pour resoudre les UUIDs et recuperer
  les skins. Il n'existe pas d'alternative pour obtenir ces donnees.
- Tout le reste (ping serveur, query, etc.) est implemente from scratch
  en Rust via les protocoles documentes.
- Le rendu 3D se fait entierement cote client (WebGL), zero service tier.
