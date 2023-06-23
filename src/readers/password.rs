use std::{fs::File, io::BufRead, io::BufReader};

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct PasswdReader {
    pub file_path: String,
    pub passwds: Vec<String>,
}

impl PasswdReader {
    pub fn new(file_path: &str) -> PasswdReader {
        PasswdReader {
            file_path: file_path.to_string(),
            passwds: vec![],
        }
    }

    pub fn collect_passwords(&mut self) -> Result<()> {
        // Open the file
        let f: File = File::open(self.file_path.clone()).context("Failed to read passwords")?;
        let r: BufReader<File> = BufReader::new(f);

        for (_i, line) in r.lines().enumerate() {
            let l = line.context("Failed to read line")?;

            if l.is_empty() {
                continue;
            }
            self.passwds.push(l);
        }
        Ok(())
    }
}
