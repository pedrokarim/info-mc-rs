use crate::types::{McColor, MotdNode, MotdStyle, code_to_color};

/// Parse a MOTD from a serde_json::Value (either a plain string or a Chat Component).
pub fn parse_motd(value: &serde_json::Value) -> Vec<MotdNode> {
    let mut nodes = Vec::new();

    match value {
        serde_json::Value::String(s) => {
            parse_legacy_string(s, &mut nodes);
        }
        serde_json::Value::Object(_) => {
            let style = MotdStyle::default();
            parse_component(value, &style, &mut nodes);
        }
        serde_json::Value::Array(arr) => {
            let style = MotdStyle::default();
            for item in arr {
                parse_component(item, &style, &mut nodes);
            }
        }
        _ => {}
    }

    nodes
}

/// Parse a legacy string with § or & formatting codes.
fn parse_legacy_string(input: &str, nodes: &mut Vec<MotdNode>) {
    let mut current_text = String::new();
    let mut style = MotdStyle::default();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if (c == '§' || c == '&') && chars.peek().is_some() {
            // Flush current text
            if !current_text.is_empty() {
                nodes.push(MotdNode {
                    text: current_text.clone(),
                    style: style.clone(),
                });
                current_text.clear();
            }

            let code = chars.next().unwrap();
            match code {
                'l' | 'L' => style.bold = true,
                'o' | 'O' => style.italic = true,
                'n' | 'N' => style.underlined = true,
                'm' | 'M' => style.strikethrough = true,
                'k' | 'K' => style.obfuscated = true,
                'r' | 'R' => style = MotdStyle::default(),
                _ => {
                    if let Some(color_name) = code_to_color(code) {
                        // Color codes reset formatting
                        style = MotdStyle {
                            color: Some(McColor::Named(color_name.to_string())),
                            ..MotdStyle::default()
                        };
                    }
                }
            }
        } else {
            current_text.push(c);
        }
    }

    if !current_text.is_empty() {
        nodes.push(MotdNode {
            text: current_text,
            style,
        });
    }
}

/// Parse a Chat Component JSON object recursively.
fn parse_component(value: &serde_json::Value, parent_style: &MotdStyle, nodes: &mut Vec<MotdNode>) {
    let obj = match value {
        serde_json::Value::String(s) => {
            // Plain string within a component array
            parse_legacy_string(s, nodes);
            return;
        }
        serde_json::Value::Object(obj) => obj,
        _ => return,
    };

    // Build style for this component, inheriting from parent
    let mut style = parent_style.clone();

    if let Some(color) = obj.get("color").and_then(|v| v.as_str()) {
        if color.starts_with('#') {
            style.color = Some(McColor::Hex(color.to_string()));
        } else {
            style.color = Some(McColor::Named(color.to_string()));
        }
    }

    if let Some(b) = obj.get("bold").and_then(|v| v.as_bool()) {
        style.bold = b;
    }
    if let Some(b) = obj.get("italic").and_then(|v| v.as_bool()) {
        style.italic = b;
    }
    if let Some(b) = obj.get("underlined").and_then(|v| v.as_bool()) {
        style.underlined = b;
    }
    if let Some(b) = obj.get("strikethrough").and_then(|v| v.as_bool()) {
        style.strikethrough = b;
    }
    if let Some(b) = obj.get("obfuscated").and_then(|v| v.as_bool()) {
        style.obfuscated = b;
    }

    // Extract text
    if let Some(text) = obj.get("text").and_then(|v| v.as_str())
        && !text.is_empty()
    {
        // The text itself might contain legacy codes
        let mut text_nodes = Vec::new();
        parse_legacy_string(text, &mut text_nodes);

        if text_nodes.is_empty() {
            nodes.push(MotdNode {
                text: text.to_string(),
                style: style.clone(),
            });
        } else {
            // Merge parent component style with parsed legacy styles
            for mut node in text_nodes {
                if node.style.color.is_none() {
                    node.style.color = style.color.clone();
                }
                if !node.style.bold {
                    node.style.bold = style.bold;
                }
                if !node.style.italic {
                    node.style.italic = style.italic;
                }
                nodes.push(node);
            }
        }
    }

    // Process "extra" children
    if let Some(extra) = obj.get("extra").and_then(|v| v.as_array()) {
        for child in extra {
            parse_component(child, &style, nodes);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_legacy_string() {
        let mut nodes = Vec::new();
        parse_legacy_string("§6§lHypixel §eNetwork", &mut nodes);

        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].text, "Hypixel ");
        assert!(nodes[0].style.bold);
        assert_eq!(nodes[1].text, "Network");
        assert!(!nodes[1].style.bold); // color code resets bold
    }

    #[test]
    fn test_parse_component() {
        let json = serde_json::json!({
            "text": "",
            "extra": [
                {"text": "Hello ", "color": "gold", "bold": true},
                {"text": "World", "color": "green"}
            ]
        });

        let nodes = parse_motd(&json);
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].text, "Hello ");
        assert!(nodes[0].style.bold);
        assert_eq!(nodes[1].text, "World");
        assert!(!nodes[1].style.bold);
    }

    #[test]
    fn test_parse_plain_string() {
        let json = serde_json::Value::String("§aHello §bWorld".to_string());
        let nodes = parse_motd(&json);
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].text, "Hello ");
        assert_eq!(nodes[1].text, "World");
    }

    #[test]
    fn test_reset_code() {
        let mut nodes = Vec::new();
        parse_legacy_string("§l§oBold Italic§rNormal", &mut nodes);

        assert_eq!(nodes.len(), 2);
        assert!(nodes[0].style.bold);
        assert!(nodes[0].style.italic);
        assert!(!nodes[1].style.bold);
        assert!(!nodes[1].style.italic);
    }
}
