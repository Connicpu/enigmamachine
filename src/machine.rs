use rotor::Rotor;
use reflector::Reflector;
use plugboard::Plugboard;

pub struct Machine {
    rotors: [Rotor; 3],
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Machine {
    pub fn new(rotors: [Rotor; 3], reflector: Reflector, plugboard: Plugboard) -> Machine {
        Machine {
            rotors: rotors,
            reflector: reflector,
            plugboard: plugboard,
        }
    }

    pub fn convert(&mut self, string: &str) -> String {
        self.format(string).chars().map(|letter| self.process(letter)).collect()
    }

    pub fn reset(&mut self) {
        for rotor in self.rotors.iter_mut() {
            rotor.reset()
        }
    }

    fn process(&mut self, mut letter: char) -> char {
        self.rotate();

        letter = self.plug(letter);
        letter = self.forward(letter);
        letter = self.reflect(letter);
        letter = self.reverse(letter);
        letter = self.plug(letter);

        letter
    }

    fn format(&self, string: &str) -> String {
        use std::ascii::AsciiExt;
        string.to_ascii_uppercase()
    }

    fn plug(&self, letter: char) -> char {
        self.plugboard.swap(letter)
    }

    fn reflect(&self, letter: char) -> char {
        self.reflector.reflect(letter)
    }

    fn forward(&self, mut letter: char) -> char {
        for rotor in self.rotors.iter() {
            letter = rotor.forward(letter);
        }
        letter
    }

    fn reverse(&self, mut letter: char) -> char {
        for rotor in self.rotors.iter().rev() {
            letter = rotor.reverse(letter);
        }
        letter
    }

    fn rotate(&mut self) {
        for rotor in self.rotors.iter_mut() {
            rotor.rotate();
            if rotor.turnover() { break; }
        }
    }
}
