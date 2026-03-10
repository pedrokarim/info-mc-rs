# Protocole Minecraft - Specification d'Implementation

Ce document detaille les protocoles reseau que la crate `mc-protocol` doit
implementer pour communiquer **directement** avec les serveurs Minecraft.

Aucun service tier n'est utilise. On parle aux serveurs nous-memes.

---

## 1. Types de donnees MC

Avant d'implementer les protocoles, il faut les types de base du protocole MC.

### VarInt (Variable-length Integer)

Entier signe sur 1 a 5 octets, encodage LEB128 modifie.

```
Valeur         | Encodage hex
---------------|------------------
0              | 0x00
1              | 0x01
127            | 0x7f
128            | 0x80 0x01
255            | 0xff 0x01
25565          | 0xdd 0xc7 0x01
2147483647     | 0xff 0xff 0xff 0xff 0x07
-1             | 0xff 0xff 0xff 0xff 0x0f
```

**Algorithme d'encodage :**
```rust
fn write_varint(mut value: i32) -> Vec<u8> {
    let mut buf = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value = ((value as u32) >> 7) as i32; // logical shift right
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 {
            break;
        }
    }
    buf
}
```

**Algorithme de decodage :**
```rust
fn read_varint(reader: &mut impl Read) -> Result<i32> {
    let mut value: i32 = 0;
    let mut position: u32 = 0;
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        let byte = byte[0];
        value |= ((byte & 0x7F) as i32) << position;
        if byte & 0x80 == 0 {
            break;
        }
        position += 7;
        if position >= 32 {
            return Err(Error::VarIntTooLong);
        }
    }
    Ok(value)
}
```

### VarLong

Meme principe que VarInt mais sur 1 a 10 octets pour un i64.

### MC String

```
[VarInt: length] [UTF-8 bytes: data]
```

- `length` = nombre d'octets UTF-8 (pas de caracteres)
- Max 32767 octets par defaut

### MC Packet

```
[VarInt: total_length] [VarInt: packet_id] [bytes: payload]
```

- `total_length` = taille de packet_id + payload en octets
- `packet_id` identifie le type de paquet

---

## 2. Server List Ping (SLP) - Java Edition

**Transport** : TCP
**Port par defaut** : 25565
**Reference** : https://wiki.vg/Server_List_Ping

### Flux complet

```
Client                                  Server
  |                                       |
  |--- TCP Connect ---------------------->|
  |                                       |
  |--- Handshake (0x00) ---------------->|
  |    protocol_version: VarInt           |
  |    server_address: String             |
  |    server_port: u16                   |
  |    next_state: VarInt (1=Status)      |
  |                                       |
  |--- Status Request (0x00) ----------->|
  |    (paquet vide, juste l'ID)          |
  |                                       |
  |<-- Status Response (0x00) -----------|
  |    json_response: String              |
  |                                       |
  |--- Ping Request (0x01) ------------->|
  |    payload: i64 (timestamp)           |
  |                                       |
  |<-- Pong Response (0x01) -------------|
  |    payload: i64 (meme valeur)         |
  |                                       |
  |--- TCP Close ------------------------>|
```

### Format du JSON de reponse

```json
{
  "version": {
    "name": "1.21.4",
    "protocol": 769
  },
  "players": {
    "max": 100,
    "online": 42,
    "sample": [
      {
        "name": "Player1",
        "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
      }
    ]
  },
  "description": {
    "text": "A Minecraft Server"
  },
  "favicon": "data:image/png;base64,<base64 encoded 64x64 PNG>",
  "enforcesSecureChat": true,
  "previewsChat": true
}
```

**Notes importantes :**
- `description` peut etre un simple string OU un objet Chat Component
  (avec `text`, `bold`, `color`, `extra`, etc.)
- `favicon` est optionnel, format `data:image/png;base64,...`
- `sample` est optionnel et souvent limite a 12 joueurs
- `protocol` permet de connaitre la version exacte du serveur
- Certains serveurs (Bungeecord) customisent fortement le JSON

### Calcul de latence

```
latence_ms = timestamp_pong_recu - timestamp_ping_envoye
```

On utilise `Instant::now()` en Rust pour une mesure precise.

### Gestion du timeout

- Timeout TCP connect : 5 secondes
- Timeout lecture reponse : 10 secondes
- Si timeout -> le serveur est considere offline

---

## 3. Query Protocol - Java Edition

**Transport** : UDP
**Port par defaut** : 25565 (meme que le port de jeu)
**Reference** : https://wiki.vg/Query
**Prerequis** : Le serveur doit avoir `enable-query=true` dans server.properties

### Handshake

```
Client -> Server:
  Magic: 0xFE 0xFD
  Type: 0x09 (handshake)
  Session ID: i32 (random, masque & 0x0F0F0F0F)

Server -> Client:
  Type: 0x09
  Session ID: i32
  Challenge Token: string (ASCII digits, null-terminated)
```

### Basic Stat

```
Client -> Server:
  Magic: 0xFE 0xFD
  Type: 0x00 (stat)
  Session ID: i32
  Challenge Token: i32

Server -> Client:
  Type: 0x00
  Session ID: i32
  Data (null-terminated strings):
    - MOTD
    - gametype (ex: "SMP")
    - map (ex: "world")
    - numplayers (ex: "5")
    - maxplayers (ex: "20")
    - hostport: u16 (little-endian)
    - hostip: string
```

### Full Stat

```
Client -> Server:
  Meme que Basic Stat, mais avec 4 octets padding (0x00 0x00 0x00 0x00)
  apres le challenge token

Server -> Client:
  Type: 0x00
  Session ID: i32
  Padding: 11 bytes ("splitnum\0\x80\0")
  K/V section (key=value pairs, double null-terminated):
    hostname, gametype, game_id, version, plugins,
    map, numplayers, maxplayers, hostport, hostip
  Padding: 10 bytes ("\0\x01player_\0\0")
  Players section (null-terminated strings, double null to end):
    "Player1\0Player2\0Player3\0\0"
```

### Parsing de la reponse plugins

Le champ `plugins` a un format special :
```
"Paper on 1.21.4: EssentialsX 2.20; WorldEdit 7.3"
 ^^^^^    ^^^^^   ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^
 software version  plugin1           plugin2
```

Format : `{software} on {version}: {plugin1}; {plugin2}; ...`

---

## 4. Bedrock Ping (Raknet Unconnected Ping)

**Transport** : UDP
**Port par defaut** : 19132
**Reference** : https://wiki.vg/Raknet_Protocol

### Unconnected Ping

```
Client -> Server:
  Packet ID: 0x01
  Time: i64 (timestamp ms)
  Magic: 00 ff ff 00 fe fe fe fe fd fd fd fd 12 34 56 78  (16 bytes)
  Client GUID: i64 (random)

Server -> Client:
  Packet ID: 0x1C
  Time: i64
  Server GUID: i64
  Magic: (16 bytes)
  String Length: u16
  Server ID String: UTF-8
```

### Format du Server ID String

Champs separes par des points-virgules :

```
Edition;MOTD_line1;Protocol;Version;Online;Max;ServerUID;MOTD_line2;Gamemode;GamemodeNumeric;Port_v4;Port_v6
```

Exemple :
```
MCPE;Dedicated Server;486;1.18.0;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133
```

| Index | Champ | Exemple |
|-------|-------|---------|
| 0 | Edition | MCPE |
| 1 | MOTD ligne 1 | Dedicated Server |
| 2 | Protocol Version | 486 |
| 3 | Version | 1.18.0 |
| 4 | Players Online | 0 |
| 5 | Max Players | 10 |
| 6 | Server Unique ID | 13253860... |
| 7 | MOTD ligne 2 | Bedrock level |
| 8 | Gamemode | Survival |
| 9 | Gamemode Numeric | 1 |
| 10 | Port IPv4 | 19132 |
| 11 | Port IPv6 | 19133 |

---

## 5. Resolution DNS SRV

Beaucoup de serveurs MC utilisent des domaines custom (ex: `play.hypixel.net`)
qui pointent vers l'IP reelle via un enregistrement DNS SRV.

### Format SRV

```
_minecraft._tcp.play.hypixel.net -> mc123.hypixel.net:25565
```

### Algorithme de resolution

```
1. Tenter de resoudre _minecraft._tcp.{domain}
2. Si SRV trouve : utiliser le target + port du SRV
3. Si pas de SRV : utiliser le domaine tel quel + port 25565
4. Resoudre le hostname final en IP (A/AAAA record)
```

### Implementation Rust

Utiliser `hickory-resolver` (anciennement trust-dns-resolver) :

```rust
use hickory_resolver::TokioAsyncResolver;
use hickory_resolver::config::*;

async fn resolve_mc_srv(domain: &str) -> Option<(String, u16)> {
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    );
    let srv_name = format!("_minecraft._tcp.{domain}");
    if let Ok(lookup) = resolver.srv_lookup(&srv_name).await {
        if let Some(record) = lookup.iter().next() {
            return Some((
                record.target().to_string().trim_end_matches('.').to_string(),
                record.port(),
            ));
        }
    }
    None
}
```

---

## 6. Detection Java vs Bedrock

Quand l'utilisateur entre une adresse sans preciser le type :

```
1. Tenter SLP (Java) sur port 25565 avec timeout 5s
2. En parallele, tenter Bedrock Ping sur port 19132 avec timeout 5s
3. Retourner le(s) resultat(s) qui repondent
4. Si les deux repondent : marquer comme "dual-stack"
```

Utiliser `tokio::select!` ou `tokio::join!` pour les requetes paralleles.

---

## 7. Mapping Protocol Version -> Version Name

Maintenir une table de correspondance :

```rust
const PROTOCOL_VERSIONS: &[(i32, &str)] = &[
    (769, "1.21.4"),
    (768, "1.21.2-1.21.3"),
    (767, "1.21-1.21.1"),
    (766, "1.20.5-1.20.6"),
    (765, "1.20.3-1.20.4"),
    (764, "1.20.2"),
    (763, "1.20-1.20.1"),
    // ... etc
];
```

Reference complete : https://wiki.vg/Protocol_version_numbers

---

## 8. Gestion d'erreurs protocole

Erreurs a gerer proprement :

| Erreur | Cause | Reponse API |
|--------|-------|-------------|
| ConnectionRefused | Serveur offline / port ferme | `{ "online": false, "error": "connection_refused" }` |
| Timeout | Serveur ne repond pas | `{ "online": false, "error": "timeout" }` |
| DnsFailure | Domaine invalide | `{ "error": "dns_resolution_failed" }` |
| InvalidResponse | Reponse malformee | `{ "error": "invalid_response" }` |
| VarIntTooLong | Protocole corrompu | `{ "error": "protocol_error" }` |
