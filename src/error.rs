#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
}

impl LoxError {
    pub fn new(line: usize, message: String) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}
