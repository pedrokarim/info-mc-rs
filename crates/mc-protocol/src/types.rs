use tokio::io::{AsyncRead, AsyncReadExt};

use crate::error::{McProtocolError, Result};

const MAX_STRING_LENGTH: usize = 32767;

// === VarInt ===

pub async fn read_varint<R: AsyncRead + Unpin>(reader: &mut R) -> Result<i32> {
    let mut value: i32 = 0;
    let mut position: u32 = 0;

    loop {
        let byte = reader.read_u8().await?;
        value |= ((byte & 0x7F) as i32) << position;

        if byte & 0x80 == 0 {
            return Ok(value);
        }

        position += 7;
        if position >= 32 {
            return Err(McProtocolError::VarIntTooLong);
        }
    }
}

pub fn write_varint(value: i32) -> Vec<u8> {
    let mut buf = Vec::with_capacity(5);
    let mut val = value as u32;

    loop {
        let mut byte = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if val == 0 {
            break;
        }
    }

    buf
}

pub fn varint_size(value: i32) -> usize {
    let mut val = value as u32;
    let mut size = 0;
    loop {
        size += 1;
        val >>= 7;
        if val == 0 {
            return size;
        }
    }
}

// === VarLong ===

pub async fn read_varlong<R: AsyncRead + Unpin>(reader: &mut R) -> Result<i64> {
    let mut value: i64 = 0;
    let mut position: u32 = 0;

    loop {
        let byte = reader.read_u8().await?;
        value |= ((byte & 0x7F) as i64) << position;

        if byte & 0x80 == 0 {
            return Ok(value);
        }

        position += 7;
        if position >= 64 {
            return Err(McProtocolError::VarLongTooLong);
        }
    }
}

pub fn write_varlong(value: i64) -> Vec<u8> {
    let mut buf = Vec::with_capacity(10);
    let mut val = value as u64;

    loop {
        let mut byte = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if val == 0 {
            break;
        }
    }

    buf
}

// === MC String ===

pub async fn read_string<R: AsyncRead + Unpin>(reader: &mut R) -> Result<String> {
    let len = read_varint(reader).await? as usize;

    if len > MAX_STRING_LENGTH {
        return Err(McProtocolError::StringTooLong {
            len,
            max: MAX_STRING_LENGTH,
        });
    }

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;

    String::from_utf8(buf).map_err(|e| McProtocolError::InvalidResponse(e.to_string()))
}

pub fn write_string(value: &str) -> Vec<u8> {
    let bytes = value.as_bytes();
    let mut buf = write_varint(bytes.len() as i32);
    buf.extend_from_slice(bytes);
    buf
}

/// Synchronous string read from a buffer (used when parsing packet payloads).
pub fn read_string_sync<R: std::io::Read>(reader: &mut R) -> Result<String> {
    let len = read_varint_sync(reader)? as usize;

    if len > MAX_STRING_LENGTH {
        return Err(McProtocolError::StringTooLong {
            len,
            max: MAX_STRING_LENGTH,
        });
    }

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf).map_err(|e| McProtocolError::InvalidResponse(e.to_string()))
}

// === MC Packet ===

pub fn build_packet(packet_id: i32, payload: &[u8]) -> Vec<u8> {
    let id_bytes = write_varint(packet_id);
    let length = id_bytes.len() + payload.len();
    let length_bytes = write_varint(length as i32);

    let mut packet = Vec::with_capacity(length_bytes.len() + length);
    packet.extend_from_slice(&length_bytes);
    packet.extend_from_slice(&id_bytes);
    packet.extend_from_slice(payload);
    packet
}

pub async fn read_packet<R: AsyncRead + Unpin>(reader: &mut R) -> Result<(i32, Vec<u8>)> {
    let length = read_varint(reader).await? as usize;

    let mut data = vec![0u8; length];
    reader.read_exact(&mut data).await?;

    let mut cursor = std::io::Cursor::new(&data);
    let packet_id = read_varint_sync(&mut cursor)?;
    let pos = cursor.position() as usize;

    Ok((packet_id, data[pos..].to_vec()))
}

/// Synchronous VarInt read for parsing from a buffer (used in read_packet).
fn read_varint_sync<R: std::io::Read>(reader: &mut R) -> Result<i32> {
    let mut value: i32 = 0;
    let mut position: u32 = 0;

    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        let byte = byte[0];

        value |= ((byte & 0x7F) as i32) << position;

        if byte & 0x80 == 0 {
            return Ok(value);
        }

        position += 7;
        if position >= 32 {
            return Err(McProtocolError::VarIntTooLong);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Helper: read varint from bytes using the sync version for simplicity
    fn decode_varint(bytes: &[u8]) -> Result<i32> {
        let mut cursor = Cursor::new(bytes);
        read_varint_sync(&mut cursor)
    }

    #[test]
    fn test_varint_encode_decode() {
        let cases: &[(i32, &[u8])] = &[
            (0, &[0x00]),
            (1, &[0x01]),
            (127, &[0x7f]),
            (128, &[0x80, 0x01]),
            (255, &[0xff, 0x01]),
            (25565, &[0xdd, 0xc7, 0x01]),
            (2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]),
            (-1, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
        ];

        for &(value, expected_bytes) in cases {
            // Test encoding
            let encoded = write_varint(value);
            assert_eq!(encoded, expected_bytes, "encoding {value}");

            // Test decoding
            let decoded = decode_varint(expected_bytes).unwrap();
            assert_eq!(decoded, value, "decoding {value}");

            // Test size
            assert_eq!(varint_size(value), expected_bytes.len(), "size of {value}");
        }
    }

    #[test]
    fn test_varint_too_long() {
        let bad = [0x80, 0x80, 0x80, 0x80, 0x80, 0x01]; // 6 bytes
        let result = decode_varint(&bad);
        assert!(matches!(result, Err(McProtocolError::VarIntTooLong)));
    }

    #[test]
    fn test_varlong_encode_decode_roundtrip() {
        let cases: &[i64] = &[0, 1, -1, 127, 128, 2147483647, -2147483648, i64::MAX, i64::MIN];

        for &value in cases {
            let encoded = write_varlong(value);
            assert!(encoded.len() <= 10, "varlong too long for {value}");

            // Decode synchronously via cursor
            let mut cursor = Cursor::new(&encoded);
            let mut decoded: i64 = 0;
            let mut position: u32 = 0;
            loop {
                let mut byte = [0u8; 1];
                std::io::Read::read_exact(&mut cursor, &mut byte).unwrap();
                let byte = byte[0];
                decoded |= ((byte & 0x7F) as i64) << position;
                if byte & 0x80 == 0 {
                    break;
                }
                position += 7;
            }
            assert_eq!(decoded, value, "roundtrip for {value}");
        }
    }

    #[test]
    fn test_string_write() {
        let encoded = write_string("Hello");
        assert_eq!(encoded, &[5, b'H', b'e', b'l', b'l', b'o']);

        let empty = write_string("");
        assert_eq!(empty, &[0]);
    }

    #[test]
    fn test_packet_build() {
        // Empty payload, packet id 0
        let pkt = build_packet(0x00, &[]);
        // length=1 (just packet_id), packet_id=0
        assert_eq!(pkt, &[1, 0]);

        // Payload [1,2,3], packet id 0
        let pkt = build_packet(0x00, &[1, 2, 3]);
        // length=4 (1 byte id + 3 payload), packet_id=0, payload
        assert_eq!(pkt, &[4, 0, 1, 2, 3]);
    }

    #[tokio::test]
    async fn test_read_packet_roundtrip() {
        let original_payload = vec![10, 20, 30, 40];
        let packet = build_packet(0x05, &original_payload);

        let mut cursor = std::io::Cursor::new(packet);
        let (id, payload) = read_packet(&mut cursor).await.unwrap();

        assert_eq!(id, 0x05);
        assert_eq!(payload, original_payload);
    }

    #[tokio::test]
    async fn test_read_string_async() {
        let data = write_string("play.hypixel.net");
        let mut cursor = std::io::Cursor::new(data);
        let result = read_string(&mut cursor).await.unwrap();
        assert_eq!(result, "play.hypixel.net");
    }

    #[tokio::test]
    async fn test_read_varint_async() {
        let data = write_varint(25565);
        let mut cursor = std::io::Cursor::new(data);
        let result = read_varint(&mut cursor).await.unwrap();
        assert_eq!(result, 25565);
    }
}
