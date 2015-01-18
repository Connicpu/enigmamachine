use std::collections::VecMap;

pub struct Plugboard {
    mappings: VecMap<char>,
}

impl Plugboard {
    pub fn new() -> Plugboard {
        Plugboard {
            mappings: VecMap::new(),
        }
    }

    pub fn from_mappings(mappings: &[(char, char)]) -> Plugboard {
        let mut board = Plugboard::new();
        for &(a, b) in mappings.iter() {
            board.plug(a, b);
        };
        board
    }

    pub fn plug(&mut self, a: char, b: char) {
        self.mappings.insert(a as usize, b);
        self.mappings.insert(b as usize, a);
    }

    pub fn swap(&self, a: char) -> char {
        let au = a as usize;
        if let Some(&b) = self.mappings.get(&au) {
            b
        } else {
            a
        }
    }
}
