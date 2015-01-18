#![allow(unstable)]

extern crate "enigmamachine" as enigma;

use std::default::Default;

use enigma::machine::Machine;
use enigma::rotor::Rotor;
use enigma::plugboard::Plugboard;

fn make_machine() -> Machine {
    let rotors = [
        Rotor::get_rotor(1, 'X'),
        Rotor::get_rotor(4, 'Q'),
        Rotor::get_rotor(3, 'K'),
    ];
    let reflector = Default::default();
    let plugboard = Plugboard::from_mappings(&[
        ('T', 'L'), ('Y', 'M'), ('H', 'X'), ('V', 'O'), ('J', 'P'),
        ('R', 'G'), ('Z', 'S'), ('Q', 'W'), ('E', 'I'), ('F', 'A'),
    ]);

    Machine::new(rotors, reflector, plugboard)
}

#[test]
fn rotor_test() {
    let mut rotor = Rotor::get_rotor(1, 'X');

    for letter in (('A' as u8) .. ('Z' as u8)) {
        rotor.reset();
        for _ in (1 .. 26) {
            assert_eq!(rotor.reverse(rotor.forward(letter as char)), letter as char);
            rotor.rotate();
        }
    }
}

#[test]
fn is_deterministic() {
    let string = "I LIKE TOAST";

    let mut machine = make_machine();
    let result1 = machine.convert(string);

    machine.reset();
    let result2 = machine.convert(string);

    assert_eq!(result1, result2);
}

#[test]
fn is_two_way() {
    let string = "I LIKE TOAST";

    let mut machine = make_machine();
    let result1 = machine.convert(string);

    machine.reset();
    let result2 = machine.convert(result1.as_slice());

    assert_eq!(result2.as_slice(), string);
}

#[test]
// The success of this test was one of the primary faults in Germany's 
// enigma machine that allowed Alan Turing to crack it.
fn no_self_characters() {
    let mut text = Vec::new();
    for _ in (1 .. 100000) {
        for letter in (('A' as u8) .. ('Z' as u8)) {
            text.push(letter);
        }
    }

    let mut machine = make_machine();
    let original = String::from_utf8(text).unwrap();
    let converted = machine.convert(original.as_slice());

    for (o, c) in original.chars().zip(converted.chars()) {
        assert!(o != c);
    }
}
