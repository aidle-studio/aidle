use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::collections::HashSet;

/// Markdownファイルから正規化された見出しのリストを抽出する。
pub fn extract_headings(content: &str) -> Vec<String> {
    let parser = Parser::new(content);
    let mut headings = Vec::new();
    let mut in_heading = false;
    let mut current_heading = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                current_heading.clear();
            }
            Event::Text(text) => {
                if in_heading {
                    current_heading.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                headings.push(normalize_heading(&current_heading));
            }
            _ => {}
        }
    }
    headings
}

/// 見出しテキストを正規化（小文字化、前後の空白削除）する。
fn normalize_heading(text: &str) -> String {
    text.trim().to_lowercase()
}

/// テンプレートファイルから特定のセクション（見出し配下）のコンテンツを抽出する。
pub fn extract_section_content(content: &str, target_heading: &str) -> String {
    let parser = Parser::new(content);
    let mut capturing = false;
    let mut section_text = String::new();
    let normalized_target = normalize_heading(target_heading);

    let mut in_heading = false;
    let mut current_heading = String::new();

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                current_heading.clear();
            }
            Event::Text(text) => {
                if in_heading {
                    current_heading.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                let normalized = normalize_heading(&current_heading);
                if normalized == normalized_target {
                    capturing = true;
                } else if capturing {
                    // 次の見出しが来たらキャプチャ終了
                    break;
                }
            }
            _ => {
                if capturing {
                    // 本文の一部を抽出（簡易的に範囲から取得）
                    section_text.push_str(&content[range]);
                }
            }
        }
    }
    section_text.trim().to_string()
}

/// テンプレートとローカルファイルを比較し、不足している見出しのリストを返す。
pub fn compare_headings(template_content: &str, local_content: &str) -> Vec<String> {
    let template_headings = extract_headings(template_content);
    let local_headings_set: HashSet<String> = extract_headings(local_content).into_iter().collect();

    template_headings
        .into_iter()
        .filter(|h| !local_headings_set.contains(h))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_headings() {
        let markdown = r#"
# Title
## Section 1
### Sub section
## Section 2
"#;
        let headings = extract_headings(markdown);
        assert_eq!(
            headings,
            vec!["title", "section 1", "sub section", "section 2"]
        );
    }

    #[test]
    fn test_compare_headings() {
        let template = r#"
# Rules
## Rule 1
## Rule 2
"#;
        let local = r#"
# Rules
## Rule 1
"#;
        let missing = compare_headings(template, local);
        assert_eq!(missing, vec!["rule 2"]);
    }

    #[test]
    fn test_extract_section_content() {
        let markdown = r#"
## Target Section
This is the content.
It has multiple lines.

## Next Section
Other content.
"#;
        let content = extract_section_content(markdown, "Target Section");
        assert!(content.contains("This is the content."));
        assert!(content.contains("It has multiple lines."));
        assert!(!content.contains("Other content."));
    }

    #[test]
    fn test_normalize_heading() {
        assert_eq!(normalize_heading("  Heading  "), "heading");
        assert_eq!(normalize_heading("Heading!"), "heading!");
    }
}
