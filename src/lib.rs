use std::io::Write;
use anyhow::{Context, Result};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<()>{
    for line in content.lines() {
        if line.contains(pattern) {
            write!(writer, "{}", line).with_context(|| format!("could not write buffer {}", pattern))?;
        }
    }
    Ok(())
}

fn answer() -> i8 {
    42
}

#[cfg(test)]
mod test {
    use crate::{answer, find_matches};

    #[test]
    fn check_answer_validity() {
        assert_eq!(answer(), 42);
    }

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_matches("this is a text\n in a test", "text", &mut result);
        assert_eq!(result, b"this is a text");
    }

}