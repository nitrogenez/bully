use passwords::PasswordGenerator;

use tracing::warn;

#[derive(Debug)]
pub struct PasswdGenerator {
    pub length: usize,
    pub passwd: Vec<char>,
    pub passwd_raw: String,
}

impl PasswdGenerator {
    pub fn new(passwd_length: Option<usize>) -> PasswdGenerator {
        warn!("Password generation is not recommended because of it's randomness");
        let l = passwd_length.unwrap_or(8);

        PasswdGenerator {
            length: l,
            passwd: vec!['a'; l],
            passwd_raw: String::new(),
        }
    }

    pub fn generate(&mut self) {
        let gen = PasswordGenerator {
            length: self.length,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: false,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        };
        self.passwd = gen.generate_one().unwrap().chars().collect();
        self.passwd_raw = self.passwd.clone().into_iter().collect();
    }
}
