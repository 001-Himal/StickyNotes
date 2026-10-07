//! Plain-text markdown formatting helpers for Sticky Note.

/// Applies keyboard-only formatting wrapper or prefix to plain text content.
pub fn apply_text_formatting(content: &str, format_type: &str) -> String {
    match format_type {
        "bold" => {
            if content.is_empty() {
                "****".to_string()
            } else if content.starts_with("**") && content.ends_with("**") && content.len() >= 4 {
                content[2..content.len() - 2].to_string()
            } else {
                format!("**{content}**")
            }
        }
        "italic" => {
            if content.is_empty() {
                "**".to_string()
            } else if content.starts_with('*')
                && content.ends_with('*')
                && content.len() >= 2
                && !(content.starts_with("**") && content.ends_with("**"))
            {
                content[1..content.len() - 1].to_string()
            } else {
                format!("*{content}*")
            }
        }
        "underline" => {
            if content.is_empty() {
                "__".to_string()
            } else if content.starts_with('_') && content.ends_with('_') && content.len() >= 2 {
                content[1..content.len() - 1].to_string()
            } else {
                format!("_{content}_")
            }
        }
        "bullet" => {
            if content.is_empty() {
                "- ".to_string()
            } else {
                content
                    .lines()
                    .map(|line| {
                        if line.trim_start().starts_with("- ") {
                            line.to_string()
                        } else {
                            format!("- {line}")
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }
        _ => content.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_formatting_wrappers() {
        assert_eq!(apply_text_formatting("test", "bold"), "**test**");
        assert_eq!(apply_text_formatting("**test**", "bold"), "test");
        assert_eq!(apply_text_formatting("test", "italic"), "*test*");
        assert_eq!(apply_text_formatting("*test*", "italic"), "test");
        assert_eq!(apply_text_formatting("test", "underline"), "_test_");
        assert_eq!(apply_text_formatting("_test_", "underline"), "test");
        assert_eq!(apply_text_formatting("item", "bullet"), "- item");
        assert_eq!(apply_text_formatting("- already bullet", "bullet"), "- already bullet");
        assert_eq!(apply_text_formatting("line 1\nline 2", "bullet"), "- line 1\n- line 2");
    }
}
