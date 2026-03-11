use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

use crate::error::{MojangError, Result, format_uuid};
use crate::types::*;

const MOJANG_API: &str = "https://api.mojang.com";
const SESSION_SERVER: &str = "https://sessionserver.mojang.com";

/// Client for the official Mojang API.
#[derive(Clone)]
pub struct MojangClient {
    http: reqwest::Client,
}

impl MojangClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::builder()
                .user_agent("MCInfo-RS/0.1")
                .build()
                .expect("failed to create HTTP client"),
        }
    }

    /// Resolve a username to a UUID.
    pub async fn username_to_uuid(&self, username: &str) -> Result<UsernameResponse> {
        validate_username(username)?;

        let url = format!("{MOJANG_API}/users/profiles/minecraft/{username}");
        let resp = self.http.get(&url).send().await?;

        match resp.status().as_u16() {
            200 => {
                let body: UsernameResponse = resp.json().await?;
                Ok(body)
            }
            204 | 404 => Err(MojangError::PlayerNotFound(username.to_string())),
            429 => Err(MojangError::RateLimited),
            status => Err(MojangError::InvalidProfile(format!(
                "unexpected status {status}"
            ))),
        }
    }

    /// Get the full profile (including skin/cape textures) for a UUID.
    /// `uuid` can be with or without hyphens.
    pub async fn get_profile(&self, uuid: &str) -> Result<SessionProfile> {
        let clean_uuid = uuid.replace('-', "");

        let url = format!("{SESSION_SERVER}/session/minecraft/profile/{clean_uuid}");
        let resp = self.http.get(&url).send().await?;

        match resp.status().as_u16() {
            200 => {
                let profile: SessionProfile = resp.json().await?;
                Ok(profile)
            }
            204 | 404 => Err(MojangError::PlayerNotFound(uuid.to_string())),
            429 => Err(MojangError::RateLimited),
            status => Err(MojangError::InvalidProfile(format!(
                "unexpected status {status}"
            ))),
        }
    }

    /// Get a complete player profile with decoded textures.
    /// Accepts a username or UUID (auto-detected).
    pub async fn get_player(&self, identifier: &str) -> Result<PlayerProfile> {
        // Determine if it's a UUID or username
        let (uuid_hex, username) = if is_uuid(identifier) {
            let clean = identifier.replace('-', "");
            let profile = self.get_profile(&clean).await?;
            (profile.id.clone(), profile.name.clone())
        } else {
            let resp = self.username_to_uuid(identifier).await?;
            (resp.id.clone(), resp.name.clone())
        };

        // Get full profile with textures
        let profile = self.get_profile(&uuid_hex).await?;
        let textures = decode_textures(&profile)?;

        let formatted_uuid = format_uuid(&uuid_hex).unwrap_or_else(|_| uuid_hex.clone());

        let skin = textures.textures.skin.map(|s| {
            let model = match s.metadata.and_then(|m| m.model) {
                Some(m) if m == "slim" => SkinModel::Slim,
                _ => SkinModel::Classic,
            };
            SkinInfo { url: s.url, model }
        });

        let cape = textures.textures.cape.map(|c| CapeInfo { url: c.url });

        Ok(PlayerProfile {
            uuid: formatted_uuid,
            username,
            skin,
            cape,
        })
    }
}

impl Default for MojangClient {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_username(username: &str) -> Result<()> {
    if username.len() < 3 || username.len() > 16 {
        return Err(MojangError::InvalidUsername(format!(
            "must be 3-16 characters, got {}",
            username.len()
        )));
    }
    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(MojangError::InvalidUsername(
            "only alphanumeric and underscore allowed".into(),
        ));
    }
    Ok(())
}

fn is_uuid(s: &str) -> bool {
    let clean = s.replace('-', "");
    clean.len() == 32 && clean.chars().all(|c| c.is_ascii_hexdigit())
}

fn decode_textures(profile: &SessionProfile) -> Result<TexturesPayload> {
    let textures_prop = profile
        .properties
        .iter()
        .find(|p| p.name == "textures")
        .ok_or_else(|| MojangError::InvalidProfile("no textures property".into()))?;

    let decoded = BASE64.decode(&textures_prop.value)?;
    let payload: TexturesPayload = serde_json::from_slice(&decoded)?;
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert!(validate_username("Dream").is_ok());
        assert!(validate_username("Technoblade").is_ok());
        assert!(validate_username("_Player_1").is_ok());
        assert!(validate_username("ab").is_err()); // too short
        assert!(validate_username("a".repeat(17).as_str()).is_err()); // too long
        assert!(validate_username("has space").is_err());
        assert!(validate_username("has.dot").is_err());
    }

    #[test]
    fn test_is_uuid() {
        assert!(is_uuid("ec70bcaf702f4bb8b48d276fa52a780c"));
        assert!(is_uuid("ec70bcaf-702f-4bb8-b48d-276fa52a780c"));
        assert!(!is_uuid("notauuid"));
        assert!(!is_uuid("Dream"));
    }

    #[test]
    fn test_decode_textures() {
        use base64::Engine;
        let json = r#"{"timestamp":1234,"profileId":"abc","profileName":"Test","textures":{"SKIN":{"url":"https://textures.minecraft.net/texture/abc123"}}}"#;
        let encoded = BASE64.encode(json);

        let profile = SessionProfile {
            id: "abc".into(),
            name: "Test".into(),
            properties: vec![ProfileProperty {
                name: "textures".into(),
                value: encoded,
                signature: None,
            }],
        };

        let textures = decode_textures(&profile).unwrap();
        assert!(textures.textures.skin.is_some());
        assert!(textures.textures.cape.is_none());
        assert_eq!(
            textures.textures.skin.unwrap().url,
            "https://textures.minecraft.net/texture/abc123"
        );
    }
}
