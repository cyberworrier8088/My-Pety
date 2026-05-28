// this module use for reading. ai can read this text


use std::fs; // import fs module

pub fn read_file(path: String) -> String { // read file func
    fs::read_to_string(path).unwrap() // read file
}
