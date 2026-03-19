# MCInfo-RS — Panneau d'Administration

## Vue d'ensemble

Le panneau admin est une interface web protégée permettant de gérer l'ensemble de la plateforme MCInfo-RS : modération du contenu, monitoring, configuration, et analytics.

**Accès** : `/admin` (frontend) → consomme `/api/v1/admin/*` (API protégée)

---

## 1. Authentification & Sécurité

### 1.1 Authentification Admin

| Élément | Détail |
|---------|--------|
| **Méthode** | Login/password avec sessions JWT |
| **Stockage** | Table `admin_users` (username, password_hash bcrypt, role, created_at) |
| **Sessions** | JWT signé (HS256), expiration 24h, refresh token optionnel |
| **2FA** | TOTP (Google Authenticator / Authy) — optionnel mais recommandé |
| **Rate limit** | 5 tentatives / 15 min par IP sur `/admin/login` |
| **Brute force** | Lockout temporaire après 10 échecs consécutifs |

### 1.2 Rôles & Permissions

| Rôle | Permissions |
|------|------------|
| **super_admin** | Tout — gestion des admins, config, purge données |
| **admin** | Modération, analytics, gestion contenu |
| **moderator** | Modération uniquement (ban, suppression likes abusifs) |
| **viewer** | Lecture seule — dashboard et analytics |

### 1.3 Audit Log

Chaque action admin est loguée :

```sql
CREATE TABLE admin_audit_log (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    admin_id    INTEGER NOT NULL,
    action      TEXT NOT NULL,        -- "ban_player", "delete_server", "update_config"...
    target_type TEXT,                 -- "player", "server", "admin_user", "config"
    target_id   TEXT,                 -- UUID, address, ou ID
    details     TEXT,                 -- JSON avec les détails de l'action
    ip_address  TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (admin_id) REFERENCES admin_users(id)
);
```

---

## 2. Dashboard

### 2.1 Vue Globale (page d'accueil admin)

**Métriques temps réel :**
- Nombre total de joueurs indexés
- Nombre total de serveurs indexés
- Nombre total de vues (aujourd'hui / cette semaine / total)
- Nombre total de likes
- Nombre de requêtes API (aujourd'hui)
- Taux de cache hit/miss

**Graphiques :**
- Courbe des recherches par jour (30 derniers jours)
- Top 10 joueurs les plus recherchés (bar chart)
- Top 10 serveurs les plus recherchés (bar chart)
- Répartition Java vs Bedrock (pie chart)
- Requêtes API par heure (heatmap)

### 2.2 Métriques Système

- Uptime de l'API
- Utilisation mémoire / CPU
- Taille de la base SQLite
- Nombre d'entrées en cache (moka)
- Latence moyenne des réponses API
- Rate limit : nombre de 429 renvoyés

---

## 3. Gestion des Joueurs

### 3.1 Liste des Joueurs

```
GET /api/v1/admin/players?page=1&per_page=50&sort=views&order=desc&search=Notch
```

**Colonnes :**
- Avatar (head render)
- Username
- UUID
- Vues
- Likes
- Premier vu / Dernier vu
- Statut (normal / banni / flagué)
- Actions

**Filtres :**
- Recherche par username / UUID
- Tri par vues, likes, date
- Filtre par statut

### 3.2 Fiche Joueur Détaillée

- Toutes les infos du profil (skin, capes, modèle)
- Historique des vues (graphique)
- Liste des IPs qui ont liké (hashées)
- Historique des usernames (si on les track au fil du temps)
- Actions : bannir, reset likes, supprimer de l'index

### 3.3 Modération Joueurs

| Action | Effet |
|--------|-------|
| **Bannir** | Le joueur n'apparaît plus dans popular/recent, son profil retourne 403 |
| **Débannir** | Restaure la visibilité |
| **Reset likes** | Remet les likes à 0 (anti-manipulation) |
| **Reset vues** | Remet les vues à 0 |
| **Supprimer** | Supprime complètement de l'index (re-indexé au prochain lookup) |
| **Flag** | Marque comme suspect pour review (n'affecte pas la visibilité) |

```sql
-- Ajout d'un champ status à la table players
ALTER TABLE players ADD COLUMN status TEXT NOT NULL DEFAULT 'active';
-- Valeurs : 'active', 'banned', 'flagged'
```

---

## 4. Gestion des Serveurs

### 4.1 Liste des Serveurs

```
GET /api/v1/admin/servers?page=1&per_page=50&sort=views&order=desc&search=hypixel
```

**Colonnes :**
- Favicon
- Adresse
- Version
- Joueurs (online/max au dernier ping)
- Édition (Java/Bedrock)
- Vues / Likes
- Dernier online
- Statut
- Actions

### 4.2 Fiche Serveur Détaillée

- Toutes les infos du dernier ping
- MOTD rendu (HTML)
- Historique des vues
- Historique uptime (si implémenté plus tard)
- Historique joueurs online (courbe)

### 4.3 Modération Serveurs

| Action | Effet |
|--------|-------|
| **Bannir** | Le serveur n'apparaît plus dans popular/recent, lookup retourne 403 |
| **Débannir** | Restaure |
| **Reset likes/vues** | Anti-manipulation |
| **Supprimer** | Supprime de l'index |
| **Flag** | Marque comme suspect |
| **Pin** | Épingle en haut des résultats populaires (promotion) |
| **Vérifier** | Badge "vérifié" (serveur officiel/connu) |

```sql
ALTER TABLE servers ADD COLUMN status TEXT NOT NULL DEFAULT 'active';
-- Valeurs : 'active', 'banned', 'flagged', 'verified', 'pinned'
```

---

## 5. Gestion des Likes

### 5.1 Anti-Manipulation

**Détection automatique :**
- Alerte si un entity reçoit > X likes en Y minutes (configurable)
- Alerte si une même IP hash like > Z entités en 1 heure
- Détection de patterns (likes en rafale depuis des IPs proches)

**Actions :**
- Purger tous les likes d'une IP hash
- Reset les likes d'une entité
- Bannir une IP hash du système de likes

### 5.2 Tableau des Likes

```
GET /api/v1/admin/likes?entity_type=player&sort=created_at&order=desc
```

- Voir tous les likes récents
- Filtrer par type (player/server)
- Identifier les patterns suspects

---

## 6. Gestion des Favoris

- Lister tous les favoris (existant, table `favorites`)
- Supprimer un favori
- Voir les stats (joueurs les plus mis en favoris)

---

## 7. Configuration Runtime

### 7.1 Paramètres Modifiables

| Paramètre | Description | Défaut |
|-----------|-------------|--------|
| `rate_limit_max` | Requêtes max par fenêtre | 100 |
| `rate_limit_window_secs` | Fenêtre du rate limit | 60 |
| `cache_ttl_server` | TTL cache serveur (secondes) | 60 |
| `cache_ttl_player` | TTL cache joueur (secondes) | 300 |
| `popular_default_limit` | Nombre de résultats par défaut pour /popular | 20 |
| `like_cooldown_secs` | Cooldown entre 2 likes d'un même IP | 5 |
| `like_alert_threshold` | Seuil d'alerte likes/minute | 50 |
| `maintenance_mode` | Active/désactive le mode maintenance | false |
| `ip_salt` | Sel pour le hash des IPs | env var |

### 7.2 Stockage Config

```sql
CREATE TABLE admin_config (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_by INTEGER,
    FOREIGN KEY (updated_by) REFERENCES admin_users(id)
);
```

### 7.3 Mode Maintenance

Quand activé :
- L'API publique retourne `503 Service Unavailable` avec un message custom
- Le panneau admin reste accessible
- Les jobs de background (si existants) sont pausés

---

## 8. Analytics & Export

### 8.1 Rapports

- **Rapport journalier** : nouvelles entrées, top recherches, total requêtes
- **Rapport hebdo** : tendances, croissance de l'index, serveurs qui montent/descendent
- **Export CSV** : liste joueurs, liste serveurs, likes (pour analyse externe)

### 8.2 Endpoints Analytics

```
GET /api/v1/admin/analytics/overview          → Métriques globales
GET /api/v1/admin/analytics/searches?days=30  → Historique recherches
GET /api/v1/admin/analytics/growth?days=30    → Croissance de l'index
GET /api/v1/admin/export/players?format=csv   → Export joueurs
GET /api/v1/admin/export/servers?format=csv   → Export serveurs
```

---

## 9. Tables SQL Admin

```sql
-- Comptes admin
CREATE TABLE admin_users (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role          TEXT NOT NULL DEFAULT 'viewer',
    totp_secret   TEXT,                              -- NULL = 2FA désactivé
    is_active     BOOLEAN NOT NULL DEFAULT 1,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    last_login_at TEXT
);

-- Sessions admin (si JWT stateless pas suffisant)
CREATE TABLE admin_sessions (
    id         TEXT PRIMARY KEY,                     -- UUID de session
    admin_id   INTEGER NOT NULL,
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (admin_id) REFERENCES admin_users(id)
);

-- Audit log (voir section 1.3)
CREATE TABLE admin_audit_log ( ... );

-- Configuration runtime (voir section 7.2)
CREATE TABLE admin_config ( ... );

-- Alertes automatiques
CREATE TABLE admin_alerts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    type        TEXT NOT NULL,        -- "like_spike", "suspicious_ip", "high_traffic"
    severity    TEXT NOT NULL,        -- "info", "warning", "critical"
    message     TEXT NOT NULL,
    entity_type TEXT,
    entity_id   TEXT,
    resolved    BOOLEAN NOT NULL DEFAULT 0,
    resolved_by INTEGER,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    resolved_at TEXT,
    FOREIGN KEY (resolved_by) REFERENCES admin_users(id)
);
```

---

## 10. Endpoints API Admin

### Auth
```
POST /api/v1/admin/login                      → Login (retourne JWT)
POST /api/v1/admin/logout                     → Invalide la session
GET  /api/v1/admin/me                         → Profil admin courant
```

### Dashboard
```
GET /api/v1/admin/dashboard                   → Métriques globales
GET /api/v1/admin/analytics/overview          → Stats détaillées
GET /api/v1/admin/analytics/searches          → Historique recherches
```

### Joueurs
```
GET    /api/v1/admin/players                  → Liste paginée
GET    /api/v1/admin/players/{uuid}           → Détail
PATCH  /api/v1/admin/players/{uuid}           → Modifier (status, reset likes/vues)
DELETE /api/v1/admin/players/{uuid}           → Supprimer de l'index
```

### Serveurs
```
GET    /api/v1/admin/servers                  → Liste paginée
GET    /api/v1/admin/servers/{address}        → Détail
PATCH  /api/v1/admin/servers/{address}        → Modifier (status, reset, pin, verify)
DELETE /api/v1/admin/servers/{address}        → Supprimer de l'index
```

### Likes
```
GET    /api/v1/admin/likes                    → Liste paginée
DELETE /api/v1/admin/likes/ip/{ip_hash}       → Purger les likes d'une IP
DELETE /api/v1/admin/likes/entity/{type}/{id} → Reset likes d'une entité
```

### Favoris
```
GET    /api/v1/admin/favorites                → Liste avec stats
DELETE /api/v1/admin/favorites/{uuid}         → Supprimer
```

### Config
```
GET    /api/v1/admin/config                   → Toute la config
PATCH  /api/v1/admin/config                   → Modifier des paramètres
```

### Admin Users (super_admin only)
```
GET    /api/v1/admin/users                    → Liste des admins
POST   /api/v1/admin/users                    → Créer un admin
PATCH  /api/v1/admin/users/{id}              → Modifier (rôle, actif/inactif)
DELETE /api/v1/admin/users/{id}              → Supprimer un admin
```

### Audit & Alertes
```
GET /api/v1/admin/audit-log                   → Historique actions
GET /api/v1/admin/alerts                      → Alertes actives
PATCH /api/v1/admin/alerts/{id}              → Résoudre une alerte
```

### Export
```
GET /api/v1/admin/export/players?format=csv
GET /api/v1/admin/export/servers?format=csv
```

---

## 11. Middleware Admin

```rust
// Middleware qui vérifie le JWT admin sur toutes les routes /api/v1/admin/*
// Extrait le rôle et le rend disponible dans les handlers
async fn admin_auth_middleware(req: Request, next: Next) -> Response {
    // 1. Extraire le token du header Authorization: Bearer <token>
    // 2. Vérifier la signature JWT
    // 3. Vérifier l'expiration
    // 4. Charger l'admin_user depuis la DB
    // 5. Vérifier que is_active = true
    // 6. Injecter AdminUser dans les extensions de la request
    // 7. next.run(req).await
}

// Guard par rôle — utilisé dans les handlers
fn require_role(admin: &AdminUser, min_role: Role) -> Result<(), ApiError> { ... }
```

---

## 12. Frontend Admin (SvelteKit)

### Structure des pages

```
/admin/login                → Page de connexion
/admin/                     → Dashboard (métriques + graphiques)
/admin/players              → Liste joueurs (table + filtres + recherche)
/admin/players/[uuid]       → Fiche joueur détaillée
/admin/servers              → Liste serveurs
/admin/servers/[address]    → Fiche serveur détaillée
/admin/likes                → Modération des likes
/admin/favorites            → Gestion des favoris
/admin/config               → Configuration runtime
/admin/users                → Gestion des admins (super_admin)
/admin/audit-log            → Historique des actions
/admin/alerts               → Alertes automatiques
/admin/export               → Exports CSV
```

### Composants Clés

- `AdminLayout` — Sidebar navigation + header avec info admin
- `DataTable` — Table paginée réutilisable (tri, filtre, recherche)
- `StatsCard` — Card avec métrique + sparkline
- `ActionModal` — Confirmation avant actions destructives
- `AuditBadge` — Badge coloré par sévérité / type d'action

---

## 13. Sécurité

| Mesure | Détail |
|--------|--------|
| **HTTPS only** | Le panneau admin doit être servi en HTTPS |
| **CORS restreint** | Les routes `/admin/*` n'acceptent que l'origine du frontend |
| **CSRF** | Token CSRF dans les formulaires (ou Double Submit Cookie) |
| **Content-Security-Policy** | Strict pour les pages admin |
| **IP Whitelist** | Option de restreindre l'accès admin à certaines IPs |
| **Session invalidation** | Déconnexion côté serveur (blacklist JWT ou sessions DB) |
| **Password policy** | Min 12 chars, complexité requise |
| **Audit complet** | Toute action loguée avec IP + timestamp |

---

## 14. Priorité d'implémentation

| Phase | Fonctionnalités |
|-------|----------------|
| **Phase 1** | Auth admin (login/JWT), dashboard basique, liste joueurs/serveurs en lecture seule |
| **Phase 2** | Actions de modération (ban, reset, delete), audit log |
| **Phase 3** | Config runtime, mode maintenance, alertes |
| **Phase 4** | Analytics avancées, graphiques, exports |
| **Phase 5** | 2FA, gestion multi-admins, IP whitelist |
