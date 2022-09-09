use anyhow::Result;
use std::io::Write;

pub const ERROR_PATTERN_EMPTY: &str = "Please pass a pattern";
pub const ERROR_NO_RESULTS: &str = "No results found";

pub fn find_matches(pattern: &str, text: &str, mut writer: impl Write) -> Result<()> {
    let mut results = Vec::new();
    for line in text.lines() {
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            results.push(line)
        }
    }

    if results.len() > 0 {
        for result in results {
            writeln!(writer, "{}", result)?;
        }
    } else {
        writeln!(writer, "{}", ERROR_NO_RESULTS)?;
    }

    Ok(())
}

#[test]

fn find_a_match() {
    let mut result = Vec::new();

    find_matches("sup", "sup\nomg\nsupagain", &mut result);
    assert_eq!(result, b"sup\nsupagain\n")
}
