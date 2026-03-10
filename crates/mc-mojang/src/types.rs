use serde::{Deserialize, Serialize};

/// Response from the Mojang username->UUID API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameResponse {
    pub name: String,
    pub id: String, // UUID without hyphens
}

/// Full player profile from the Mojang session server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProfile {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub properties: Vec<ProfileProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String, // Base64-encoded JSON
    #[serde(default)]
    pub signature: Option<String>,
}

/// Decoded textures property from a player profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TexturesPayload {
    pub timestamp: Option<u64>,
    #[serde(rename = "profileId")]
    pub profile_id: Option<String>,
    #[serde(rename = "profileName")]
    pub profile_name: Option<String>,
    pub textures: Textures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Textures {
    #[serde(rename = "SKIN")]
    pub skin: Option<TextureEntry>,
    #[serde(rename = "CAPE")]
    pub cape: Option<TextureEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureEntry {
    pub url: String,
    #[serde(default)]
    pub metadata: Option<TextureMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureMetadata {
    pub model: Option<String>, // "slim" for alex-style, absent for classic steve-style
}

/// Our processed player profile, ready for API responses.
#[derive(Debug, Clone, Serialize)]
pub struct PlayerProfile {
    pub uuid: String,
    pub username: String,
    pub skin: Option<SkinInfo>,
    pub cape: Option<CapeInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkinInfo {
    pub url: String,
    pub model: SkinModel,
}

#[derive(Debug, Clone, Serialize)]
pub struct CapeInfo {
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SkinModel {
    Classic,
    Slim,
}
