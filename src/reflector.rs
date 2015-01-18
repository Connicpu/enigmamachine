use std::default::Default;
use num::integer::Integer;

static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

#[derive(Copy)]
pub struct Reflector {
    mappings: [char; 26],
    position: u8
}

impl Reflector {
    pub fn new(mappings: [char; 26], position: u8) -> Reflector {
        Reflector {
            mappings: mappings,
            position: position,
        }
    }

    pub fn reflect(&self, letter: char) -> char {
        if let Some(index) = self.mappings.position_elem(&letter) {
            ALPHABET[(index + self.position as usize).mod_floor(&ALPHABET.len())]
        } else {
            letter
        }
    }
}

impl Default for Reflector {
    fn default() -> Reflector {
        Reflector::new([
            'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P',
            'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B',
            'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T'
        ], 0)
    }
}
