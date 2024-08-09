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
    use std::error::Error;
    use assert_fs::prelude::*;
    use crate::{answer, find_matches};
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command; // Run programs

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

    #[test]
    fn content_in_file() -> Result<(), Box<dyn Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let mut cmd = Command::cargo_bin("grrs")?;
        cmd.arg("test").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("A test\nAnother test"));
        Ok(())
    }

    // #[test]
    // fn empty_string_as_pattern() -> Result<(), dyn Error> {
    //     let file = NamedTempFile::new("sample.text")?;
    //     file.write_str("A test\nActual content\nMore content\nAnother test")?;
    //
    //     let mut cmd = Command::cargo_bin("grrs")?;
    //     cmd.arg("test").arg(file.path());
    //     cmd.assert()
    //         .success()
    //         .stdout(predicate::str::contains(""));
    //     Ok(())
    // }

}