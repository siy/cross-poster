/// Clean AI artifacts from text
///
/// Removes Unicode emojis, smart quotes, dashes, and other AI-generated formatting
pub fn clean_ai_artifacts(text: &str) -> String {
    let mut result = text.to_string();

    // Remove Unicode emojis
    result = remove_emojis(&result);

    // Replace typographic characters
    result = replace_typography(&result);

    // Remove special whitespace and zero-width characters
    result = clean_whitespace(&result);

    result
}

/// Remove Unicode emoji characters
fn remove_emojis(text: &str) -> String {
    text.chars()
        .filter(|&c| {
            let code = c as u32;
            // Emoji ranges
            let is_emoji = matches!(code,
                0x1F600..=0x1F64F | // Emoticons
                0x1F300..=0x1F5FF | // Misc Symbols and Pictographs
                0x1F680..=0x1F6FF | // Transport and Map
                0x1F1E0..=0x1F1FF | // Regional Indicators
                0x2600..=0x26FF   | // Misc symbols
                0x2700..=0x27BF   | // Dingbats
                0xFE00..=0xFE0F   | // Variation Selectors
                0x1F900..=0x1F9FF | // Supplemental Symbols and Pictographs
                0x1F018..=0x1F270 | // Various asian characters
                0x238C..=0x2454   | // Misc items
                0x20D0..=0x20FF     // Combining Diacritical Marks for Symbols
            );
            !is_emoji
        })
        .collect()
}

/// Replace typographic characters with ASCII equivalents
fn replace_typography(text: &str) -> String {
    text
        // Em dash → double hyphen
        .replace('\u{2014}', "--")
        .replace("—", "--")
        // En dash → single hyphen
        .replace('\u{2013}', "-")
        .replace("–", "-")
        // Smart double quotes → straight quotes
        .replace(['\u{201C}', '\u{201D}'], "\"")
        // Smart single quotes → straight apostrophes
        .replace(['\u{2018}', '\u{2019}'], "'")
        // Ellipsis → three dots
        .replace('\u{2026}', "...")
}

/// Clean special whitespace and zero-width characters
fn clean_whitespace(text: &str) -> String {
    text.chars()
        .filter(|&c| {
            // Filter out problematic whitespace
            !matches!(
                c,
                '\u{00A0}' | // Non-breaking space
                '\u{200B}' | // Zero-width space
                '\u{200C}' | // Zero-width non-joiner
                '\u{200D}' | // Zero-width joiner
                '\u{FEFF}' // Zero-width no-break space
            )
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_emojis() {
        let text = "Hello 👋 World 🌍!";
        let cleaned = remove_emojis(text);
        assert_eq!(cleaned, "Hello  World !");
    }

    #[test]
    fn test_replace_em_dash() {
        let text = "This is an em dash — right here.";
        let cleaned = replace_typography(text);
        assert_eq!(cleaned, "This is an em dash -- right here.");
    }

    #[test]
    fn test_replace_en_dash() {
        let text = "Range: 1–10";
        let cleaned = replace_typography(text);
        assert_eq!(cleaned, "Range: 1-10");
    }

    #[test]
    fn test_replace_smart_quotes() {
        let text = "\u{201C}Hello\u{201D} and \u{2018}world\u{2019}";
        let cleaned = replace_typography(text);
        assert_eq!(cleaned, "\"Hello\" and 'world'");
    }

    #[test]
    fn test_replace_ellipsis() {
        let text = "Wait…";
        let cleaned = replace_typography(text);
        assert_eq!(cleaned, "Wait...");
    }

    #[test]
    fn test_clean_zero_width_characters() {
        let text = "Hello\u{200B}World\u{FEFF}!";
        let cleaned = clean_whitespace(text);
        assert_eq!(cleaned, "HelloWorld!");
    }

    #[test]
    fn test_clean_ai_artifacts_comprehensive() {
        let text =
            "Hello 👋 — this is a \u{201C}test\u{201D} with \u{2018}quotes\u{2019} and … ellipsis";
        let cleaned = clean_ai_artifacts(text);
        assert_eq!(
            cleaned,
            "Hello  -- this is a \"test\" with 'quotes' and ... ellipsis"
        );
    }

    #[test]
    fn test_clean_ai_artifacts_preserves_normal_text() {
        let text = "Normal text without any special characters.";
        let cleaned = clean_ai_artifacts(text);
        assert_eq!(cleaned, text);
    }
}
