use std::collections::HashMap;
pub mod compare;
pub mod perft;
pub mod plan;

#[derive(Debug, Default)]
pub struct Args {
    pub positional: Vec<String>,
    pub flags: HashMap<String, String>,
}

impl Args {
    pub fn parse(args: &str) -> Self {
        let mut positional = Vec::new();
        let mut flags = HashMap::new();

        let mut tokens = args.split_whitespace().peekable();
        while let Some(token) = tokens.next() {
            if token.starts_with("--") {
                if let Some(&value) = tokens.peek() {
                    flags.insert(token.to_string(), value.to_string());
                    tokens.next();
                }
            } else {
                positional.push(token.to_string());
            }
        }

        Self { positional, flags }
    }

    pub fn get_flag(&self, name: &str) -> Option<&str> {
        self.flags.get(name).map(|s| s.as_str())
    }
}
