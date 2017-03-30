extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{ BufWriter, Write };
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("flip_char.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "/// Compile time generated lookup table for flipped characters.\n").unwrap();
    write!(&mut file, "/// \n").unwrap();
    write!(&mut file, "pub static FLIPPED: phf::Map<char, char> = ").unwrap();

    let mut m = phf_codegen::Map::new();

    m.entry('a',        "'\u{0250}'");
    m.entry('b',        "'q'");
    m.entry('c',        "'\u{0254}'");
    m.entry('d',        "'p'");
    m.entry('e',        "'\u{01DD}'");
    m.entry('f',        "'\u{025F}'");
    m.entry('g',        "'\u{0183}'");
    m.entry('h',        "'\u{0265}'");
    m.entry('i',        "'\u{0131}'");
    m.entry('j',        "'\u{027E}'");
    m.entry('k',        "'\u{029E}'");
    m.entry('l',        "'|'");
    m.entry('m',        "'\u{026F}'");
    m.entry('n',        "'u'");
    m.entry('o',        "'o'");
    m.entry('p',        "'d'");
    m.entry('q',        "'b'");
    m.entry('r',        "'\u{0279}'");
    m.entry('s',        "'s'");
    m.entry('t',        "'\u{0287}'");
    m.entry('u',        "'n'");
    m.entry('v',        "'\u{028C}'");
    m.entry('w',        "'\u{028D}'");
    m.entry('x',        "'x'");
    m.entry('y',        "'\u{028E}'");
    m.entry('z',        "'z'");
    m.entry('A',        "'\u{0250}'");
    m.entry('B',        "'q'");
    m.entry('C',        "'\u{0254}'");
    m.entry('D',        "'p'");
    m.entry('E',        "'\u{01DD}'");
    m.entry('F',        "'\u{025F}'");
    m.entry('G',        "'\u{0183}'");
    m.entry('H',        "'\u{0265}'");
    m.entry('I',        "'\u{0131}'");
    m.entry('J',        "'\u{027E}'");
    m.entry('K',        "'\u{029E}'");
    m.entry('L',        "'|'");
    m.entry('M',        "'\u{026F}'");
    m.entry('N',        "'u'");
    m.entry('O',        "'o'");
    m.entry('P',        "'d'");
    m.entry('Q',        "'b'");
    m.entry('R',        "'\u{0279}'");
    m.entry('S',        "'s'");
    m.entry('T',        "'\u{0287}'");
    m.entry('U',        "'n'");
    m.entry('V',        "'\u{028C}'");
    m.entry('W',        "'\u{028D}'");
    m.entry('X',        "'x'");
    m.entry('Y',        "'\u{028E}'");
    m.entry('Z',        "'z'");
    m.entry('.',        "'\u{02D9}'");
    m.entry('[',        "']'");
    m.entry('\'',       "','");
    m.entry(',',        "'\\''");
    m.entry('(',        "')'");
    m.entry('{',        "'}'");
    m.entry('?',        "'\u{00BF}'");
    m.entry('!',        "'\u{00A1}'");
    m.entry('"',        "','");
    m.entry('<',        "'>'");
    m.entry('_',        "'\u{203E}'");
    m.entry(';',        "'\u{061B}'");
    m.entry('\u{203F}', "'\u{2040}'");
    m.entry('\u{2045}', "'\u{2046}'");
    m.entry('\u{2234}', "'\u{2235}'");
    m.entry('\r',       "'\\n'");
    m.entry(' ',        "' '");
    m.entry('-',        "'-'");

    m.entry('0',        "'0'");
    m.entry('9',        "'6'");
    m.entry('7',        "'ㄥ'");
    m.entry('6',        "'9'");
    m.entry('3',        "'Ɛ'");
    m.entry('1',        "'⇂'");

    m.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}
