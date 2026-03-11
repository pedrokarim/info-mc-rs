use axum::Json;
use serde_json::{Value, json};

/// GET /api/docs — OpenAPI 3.0 spec (minimal, no external deps).
pub async fn api_docs() -> Json<Value> {
    Json(json!({
        "openapi": "3.0.3",
        "info": {
            "title": "MCInfo-RS API",
            "description": "Minecraft server & player intelligence. Toutes les données obtenues directement (SLP, Query, Bedrock Raknet, Mojang API officielle). Aucun service tier.",
            "version": env!("CARGO_PKG_VERSION")
        },
        "servers": [
            { "url": "http://127.0.0.1:3001", "description": "Local dev" }
        ],
        "paths": {
            "/health": {
                "get": {
                    "summary": "Health check",
                    "operationId": "health",
                    "responses": {
                        "200": {
                            "description": "API opérationnelle",
                            "content": {
                                "application/json": {
                                    "example": { "status": "ok", "version": env!("CARGO_PKG_VERSION") }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/server/{address}": {
                "get": {
                    "summary": "Snapshot d'un serveur Minecraft",
                    "description": "Ping direct (SLP TCP / Bedrock UDP). Résolution DNS SRV automatique. Cache 60 s.",
                    "operationId": "getServer",
                    "parameters": [
                        {
                            "name": "address",
                            "in": "path",
                            "required": true,
                            "description": "IP ou domaine, port optionnel (ex: play.hypixel.net, 1.2.3.4:25565)",
                            "schema": { "type": "string" }
                        },
                        {
                            "name": "type",
                            "in": "query",
                            "required": false,
                            "description": "Edition forcée",
                            "schema": { "type": "string", "enum": ["auto", "java", "bedrock"], "default": "auto" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Snapshot serveur (online: true/false selon disponibilité)",
                            "content": {
                                "application/json": {
                                    "example": {
                                        "online": true,
                                        "address": {
                                            "hostname": "play.hypixel.net",
                                            "ip": "172.65.128.35",
                                            "port": 25565,
                                            "srv_record": true
                                        },
                                        "version": { "name": "Requires MC 1.8 / 1.21", "protocol": 47 },
                                        "players": { "online": 42389, "max": 200000 },
                                        "motd": {
                                            "raw": "§6§lHypixel Network §e[1.8-1.21]",
                                            "clean": "Hypixel Network [1.8-1.21]",
                                            "html": "<span style=\"color:#FFAA00;font-weight:bold\">Hypixel Network</span>"
                                        },
                                        "favicon": "data:image/png;base64,...",
                                        "latency_ms": 34,
                                        "edition": "java",
                                        "retrieved_at": "2026-03-11T12:00:00Z"
                                    }
                                }
                            }
                        },
                        "422": { "description": "Adresse invalide ou DNS introuvable" },
                        "504": { "description": "Timeout" }
                    }
                }
            },
            "/api/v1/player/{identifier}": {
                "get": {
                    "summary": "Profil joueur Minecraft",
                    "description": "Résolution UUID via api.mojang.com, textures via sessionserver.mojang.com. Cache 5 min.",
                    "operationId": "getPlayer",
                    "parameters": [
                        {
                            "name": "identifier",
                            "in": "path",
                            "required": true,
                            "description": "Username (3–16 chars) ou UUID",
                            "schema": { "type": "string" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Profil joueur",
                            "content": {
                                "application/json": {
                                    "example": {
                                        "uuid": "069a79f4-44e9-4726-a5be-fca90e38aaf5",
                                        "username": "Notch",
                                        "skin": {
                                            "url": "http://textures.minecraft.net/texture/...",
                                            "model": "classic"
                                        },
                                        "cape": null,
                                        "retrieved_at": "2026-03-11T12:00:00Z"
                                    }
                                }
                            }
                        },
                        "400": { "description": "Username invalide" },
                        "404": { "description": "Joueur introuvable" }
                    }
                }
            },
            "/api/v1/render/{identifier}": {
                "get": {
                    "summary": "Rendu 2D du skin (PNG)",
                    "description": "Génération serveur-side via la crate image. Nearest-neighbor scaling.",
                    "operationId": "renderSkin2d",
                    "parameters": [
                        {
                            "name": "identifier",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string" }
                        },
                        {
                            "name": "type",
                            "in": "query",
                            "description": "Type de rendu",
                            "schema": { "type": "string", "enum": ["face", "head", "full"], "default": "face" }
                        },
                        {
                            "name": "size",
                            "in": "query",
                            "description": "Taille en pixels (8–512)",
                            "schema": { "type": "integer", "minimum": 8, "maximum": 512, "default": 64 }
                        },
                        {
                            "name": "overlay",
                            "in": "query",
                            "description": "Inclure la couche overlay (chapeau, veste…)",
                            "schema": { "type": "boolean", "default": true }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Image PNG",
                            "content": { "image/png": {} }
                        }
                    }
                }
            },
            "/api/v1/render3d/{identifier}": {
                "get": {
                    "summary": "Rendu 3D du skin (PNG, offscreen wgpu)",
                    "description": "Rendu GPU offscreen via wgpu. Modèle complet avec UV mapping, shading Lambert.",
                    "operationId": "renderSkin3d",
                    "parameters": [
                        {
                            "name": "identifier",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string" }
                        },
                        {
                            "name": "theta",
                            "in": "query",
                            "description": "Rotation horizontale en degrés (défaut: 30)",
                            "schema": { "type": "number", "default": 30 }
                        },
                        {
                            "name": "phi",
                            "in": "query",
                            "description": "Inclinaison verticale en degrés (défaut: 21)",
                            "schema": { "type": "number", "default": 21 }
                        },
                        {
                            "name": "width",
                            "in": "query",
                            "description": "Largeur en pixels (8–512, défaut: 240)",
                            "schema": { "type": "integer", "minimum": 8, "maximum": 512, "default": 240 }
                        },
                        {
                            "name": "height",
                            "in": "query",
                            "description": "Hauteur en pixels (8–512, défaut: 360)",
                            "schema": { "type": "integer", "minimum": 8, "maximum": 512, "default": 360 }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Image PNG du rendu 3D",
                            "content": { "image/png": {} }
                        },
                        "404": { "description": "Joueur introuvable ou sans skin" }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "ErrorResponse": {
                    "type": "object",
                    "properties": {
                        "error": { "type": "string" },
                        "message": { "type": "string" }
                    }
                }
            }
        }
    }))
}
