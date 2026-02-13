pub struct Scanner{}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {}
    }

    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Token {}
