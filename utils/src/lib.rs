use std::{fs::File, io::Read};

/// A collection of utility functions for handling Inputs
pub struct AOCInput {
    pub date: &'static str,
}

impl Into<String> for AOCInput {
    fn into(self) -> String {
        let mut input: String = String::new();
        let mut file = File::open(format!("./{}/input.txt", self.date)).expect("Missing Input");
        file.read_to_string(&mut input)
            .expect("Failed to read Input");
        input
    }
}

impl Into<Vec<u8>> for AOCInput {
    fn into(self) -> Vec<u8> {
        let mut input: Vec<u8> = Vec::new();
        let mut file = File::open(format!("./{}/input.txt", self.date)).expect("Missing Input");
        file.read_to_end(&mut input)
            .expect("Failed to read Input");
        input
    }
}
