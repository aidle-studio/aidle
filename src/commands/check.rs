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

/// テンプレートファイルから特定のセクション（見出し行そのものを含む）全体を抽出する。
pub fn extract_full_section(content: &str, target_heading: &str) -> String {
    let parser = Parser::new(content);
    let mut capturing = false;
    let mut start_offset = 0;
    let mut end_offset = content.len();
    let normalized_target = normalize_heading(target_heading);

    let mut in_heading = false;
    let mut current_heading = String::new();

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                current_heading.clear();

                if capturing {
                    // We were capturing and hit the NEXT heading. Stop here.
                    end_offset = range.start;
                    break;
                }
            }
            Event::Text(text) => {
                if in_heading {
                    current_heading.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                let normalized = normalize_heading(&current_heading);
                if normalized == normalized_target && !capturing {
                    capturing = true;
                    // Note: range.start for Event::Start(Heading) is unfortunately lost unless we saved it.
                    // But we can approximate by finding the last '#' before range.start
                    start_offset = content[..range.start].rfind('#').unwrap_or(range.start);
                }
            }
            _ => {}
        }
    }

    if capturing {
        content[start_offset..end_offset].trim_end().to_string()
    } else {
        String::new()
    }
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

/// テンプレート内で `target_heading` の直前にある見出しを特定する
pub fn find_preceding_heading(content: &str, target_heading: &str) -> Option<String> {
    let headings = extract_headings(content);
    let normalized_target = normalize_heading(target_heading);

    let mut prev = None;
    for h in headings {
        if h == normalized_target {
            return prev;
        }
        prev = Some(h);
    }
    None
}

/// ローカルファイルの `target_heading` セクションの終了位置（次の見出しの開始位置、またはEOF）を見つけ、
/// そこに `new_content` を挿入して新しいMarkdown文字列を返す。
/// target_heading が None の場合、または見つからない場合は末尾に追記する。
pub fn insert_after_section(
    local_content: &str,
    target_heading: Option<&str>,
    new_content: &str,
) -> String {
    let mut result = String::new();

    // If no target heading, append to EOF safely
    let Some(target) = target_heading else {
        let trimmed = local_content.trim_end();
        if trimmed.is_empty() {
            return format!("{}\n", new_content.trim());
        }
        return format!("{}\n\n{}\n", trimmed, new_content.trim());
    };

    let normalized_target = normalize_heading(target);

    let parser = Parser::new(local_content);
    let mut in_target_section = false;
    let mut insert_offset = None;

    let mut in_heading = false;
    let mut current_heading = String::new();

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                current_heading.clear();

                // If we were in the target section and found a NEW heading, this is where we insert
                if in_target_section {
                    insert_offset = Some(range.start);
                    break;
                }
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
                    in_target_section = true;
                }
            }
            _ => {}
        }
    }

    let trimmed_local = local_content.trim_end();

    if let Some(offset) = insert_offset {
        // Insert right before the next heading
        let before = &local_content[..offset];
        let after = &local_content[offset..];

        let before_trimmed = before.trim_end();
        result.push_str(before_trimmed);
        result.push_str("\n\n");
        result.push_str(new_content.trim());
        result.push_str("\n\n");
        result.push_str(after);
    } else if in_target_section {
        // Target section is the last section, append to EOF
        result.push_str(trimmed_local);
        result.push_str("\n\n");
        result.push_str(new_content.trim());
        result.push('\n');
    } else {
        // Target heading not found in local file, append to EOF
        result.push_str(trimmed_local);
        if !trimmed_local.is_empty() {
            result.push_str("\n\n");
        }
        result.push_str(new_content.trim());
        result.push('\n');
    }

    result
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

    #[test]
    fn test_find_preceding_heading() {
        let markdown = r#"
# Title
## Section 1
## Target Section
## Section 3
"#;
        assert_eq!(
            find_preceding_heading(markdown, "Target Section").as_deref(),
            Some("section 1")
        );
        assert_eq!(find_preceding_heading(markdown, "Title"), None);
        assert_eq!(
            find_preceding_heading(markdown, "Section 3").as_deref(),
            Some("target section")
        );
    }

    #[test]
    fn test_insert_after_section_middle() {
        let local = "## Sec 1\nContent 1\n## Sec 3\nContent 3";
        let new_content = "## Sec 2\nContent 2";

        let result = insert_after_section(local, Some("Sec 1"), new_content);
        let expected = "## Sec 1\nContent 1\n\n## Sec 2\nContent 2\n\n## Sec 3\nContent 3";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_insert_after_section_eof() {
        let local = "## Sec 1\nContent 1";
        let new_content = "## Sec 2\nContent 2";

        let result = insert_after_section(local, Some("Sec 1"), new_content);
        let expected = "## Sec 1\nContent 1\n\n## Sec 2\nContent 2\n";
        assert_eq!(result, expected);
    }
}
