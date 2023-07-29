use std::collections::HashMap;
use lazy_static::lazy_static;




lazy_static! {
    pub static ref ASCII_TO_ERIKA: HashMap<char, &'static [u8; 1]> = {
        let mut map = HashMap::new();

        // Control chars
        map.insert('\u{0008}', b"\x72");
        map.insert('\t', b"\x79");
        map.insert('\n', b"\x77");
        map.insert('\r', b"\x78");

        // Punctuation
        map.insert(' ', b"\x71");
        map.insert('!', b"\x42");
        map.insert('"', b"\x43");
        map.insert('#', b"\x41");
        map.insert('$', b"\x48");
        map.insert('%', b"\x04");
        map.insert('&', b"\x02");
        map.insert('\'', b"\x17");
        map.insert('(', b"\x1D");
        map.insert(')', b"\x1F");
        map.insert('*', b"\x1B");
        map.insert('+', b"\x25");
        map.insert(',', b"\x64");
        map.insert('-', b"\x62");
        map.insert('.', b"\x63");
        map.insert('/', b"\x40");

        // Digits
        map.insert('0', b"\x0D");
        map.insert('1', b"\x11");
        map.insert('2', b"\x10");
        map.insert('3', b"\x0F");
        map.insert('4', b"\x0E");
        map.insert('5', b"\x0C");
        map.insert('6', b"\x0B");
        map.insert('7', b"\x0A");
        map.insert('8', b"\x09");
        map.insert('9', b"\x08");

        // More punctuation
        map.insert(':', b"\x13");
        map.insert(';', b"\x3B");
        map.insert('=', b"\x2E");
        map.insert('?', b"\x35");

        // Upper case letters
        map.insert('A', b"\x30");
        map.insert('B', b"\x18");
        map.insert('C', b"\x20");
        map.insert('D', b"\x14");
        map.insert('E', b"\x34");
        map.insert('F', b"\x3E");
        map.insert('G', b"\x1C");
        map.insert('H', b"\x12");
        map.insert('I', b"\x21");
        map.insert('J', b"\x32");
        map.insert('K', b"\x24");
        map.insert('L', b"\x2C");
        map.insert('M', b"\x16");
        map.insert('N', b"\x2A");
        map.insert('O', b"\x1E");
        map.insert('P', b"\x2F");
        map.insert('Q', b"\x1A");
        map.insert('R', b"\x36");
        map.insert('S', b"\x33");
        map.insert('T', b"\x37");
        map.insert('U', b"\x28");
        map.insert('V', b"\x22");
        map.insert('W', b"\x2D");
        map.insert('X', b"\x26");
        map.insert('Y', b"\x31");
        map.insert('Z', b"\x38");

        // Punctuation
        // map.insert('^', b"\x19\x71"); //TODO
        map.insert('_', b"\x01");
        // map.insert('`', b"\x2B\x71"); //TODO

        // Lower case letters
        map.insert('a', b"\x61");
        map.insert('b', b"\x4E");
        map.insert('c', b"\x57");
        map.insert('d', b"\x53");
        map.insert('e', b"\x5A");
        map.insert('f', b"\x49");
        map.insert('g', b"\x60");
        map.insert('h', b"\x55");
        map.insert('i', b"\x05");
        map.insert('j', b"\x4B");
        map.insert('k', b"\x50");
        map.insert('l', b"\x4D");
        map.insert('m', b"\x4A");
        map.insert('n', b"\x5C");
        map.insert('o', b"\x5E");
        map.insert('p', b"\x5B");
        map.insert('q', b"\x52");
        map.insert('r', b"\x59");
        map.insert('s', b"\x58");
        map.insert('t', b"\x56");
        map.insert('u', b"\x5D");
        map.insert('v', b"\x4F");
        map.insert('w', b"\x4C");
        map.insert('x', b"\x5F");
        map.insert('y', b"\x51");
        map.insert('z', b"\x54");

        // Special chars
        map.insert('|', b"\x27");
        map.insert('£', b"\x06");
        map.insert('§', b"\x3D");
        // map.insert('¨', b"\x03\x71"); //TODO
        map.insert('°', b"\x39");
        map.insert('²', b"\x15");
        map.insert('³', b"\x23");

        // Umlauts, accents
        map.insert('Ä', b"\x3F");
        map.insert('Ö', b"\x3C");
        map.insert('Ü', b"\x3A");
        map.insert('ß', b"\x47");
        map.insert('ä', b"\x65");
        map.insert('ç', b"\x45");
        map.insert('è', b"\x46");
        map.insert('é', b"\x44");
        map.insert('ö', b"\x66");
        map.insert('ü', b"\x67");
        // map.insert('´', b"\x29\x71"); //TODO
        map.insert('μ', b"\x07");

        map
    };
}

const COMBINED_CHARS: [&str; 5] = ["^", "`", "¨", "´", "€"];

lazy_static! {
    pub static ref COMBINING_DIACRITICS: HashMap<char, &'static [u8;1]> = {
        let mut map = HashMap::new();
        map.insert('\u{0300}', b"\x2B");
        map.insert('\u{0301}', b"\x29");
        map.insert('\u{0302}', b"\x19");
        map.insert('\u{0308}', b"\x03");
        map.insert('\u{030a}', b"\x39");
        map
    };
}

struct Direction;


impl Direction {
    const RIGHT: &'static [u8] = b"\x73";
    const LEFT: &'static [u8] = b"\x74";
    const UP: &'static [u8] = b"\x76";
    const DOWN: &'static [u8] = b"\x75";
}

const LINE_FEED: &'static [u8] = b"\x9F";

struct MicroStep;

impl MicroStep {
    const LEFT_RIGHT: &'static [u8] = b"\xA5";
    const UP: &'static [u8] = b"\x82";
    const DOWN: &'static [u8] = b"\x81";
}