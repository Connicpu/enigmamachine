use num::integer::Integer;

static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

#[derive(Copy)]
pub struct Rotor {
    mappings: [char; 26],
    turnover: i16,
    position: i16,
    offset: i16,
}

impl Rotor {
    pub fn new(mappings: [char; 26], turnover: u8, position: u8) -> Rotor {
        Rotor {
            mappings: mappings,
            turnover: turnover as i16,
            position: position as i16,
            offset: 0,
        }
    }

    pub fn rotate(&mut self) {
        self.offset += 1;
        self.offset %= ALPHABET.len() as i16;
    }

    pub fn turnover(&self) -> bool {
        ((self.position + self.offset) % ALPHABET.len() as i16) == self.turnover
    }

    pub fn forward(&self, letter: char) -> char {
        if let Some(index) = ALPHABET.position_elem(&letter) {
            let index = index as i16;
            let index = index + self.position + self.offset;
            let index = index.mod_floor(&(self.mappings.len() as i16));
            self.mappings[index as usize]
        } else {
            letter
        }
    }

    pub fn reverse(&self, letter: char) -> char {
        if let Some(index) = self.mappings.position_elem(&letter) {
            let index = index as i16;
            let index = index - (self.position + self.offset);
            let index = index.mod_floor(&(ALPHABET.len() as i16));
            ALPHABET[index as usize]
        } else {
            letter
        }
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn get_rotor(rotor_number: usize, position: char) -> Rotor {
        let map = ROTOR_MAPS[rotor_number - 1];
        let turnover = map.mappings.position_elem(&map.turnover).unwrap() as u8;
        let position = ALPHABET.position_elem(&position).unwrap() as u8;
        Rotor::new(map.mappings, turnover, position)
    }
}

#[derive(Copy)]
struct RotorMap {
    mappings: [char; 26],
    turnover: char,
}

static ROTOR_MAPS: [RotorMap; 5] = [
    RotorMap {
        mappings: [
            'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V',
            'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U',
            'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J'
        ],
        turnover: 'R',
    },
    RotorMap {
        mappings: [
            'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X',
            'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G',
            'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'
        ],
        turnover: 'F',
    },
    RotorMap {
        mappings: [
            'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R',
            'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W',
            'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O'
        ],
        turnover: 'W',
    },
    RotorMap {
        mappings: [
            'E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y',
            'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F',
            'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B'
        ],
        turnover: 'K',
    },
    RotorMap {
        mappings: [
            'V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U',
            'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A', 'W',
            'M', 'J', 'Q', 'O', 'F', 'E', 'C', 'K'
        ],
        turnover: 'A',
    },
];
