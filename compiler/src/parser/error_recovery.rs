pub struct ErrorRecoveryStream<'a> {
    pub input: &'a str,
    pub cursor: usize,
}

impl<'a> ErrorRecoveryStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }

    pub fn resync_to_next_statement(&mut self) -> Option<usize> {
        let remaining = &self.input[self.cursor..];
        if let Some(pos) = remaining.find(';') {
            self.cursor += pos + 1;
            Some(self.cursor)
        } else {
            None
        }
    }
}
