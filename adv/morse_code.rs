use std::collections::HashMap;
use std::io;

const UNKNOWN_CHARACTER: &str = "........";
const _UNKNOWN_MORSE_CHARACTER: &str = "_";

pub fn encode(message: &str) -> String {
    let dictionary = _morse_dictionary();
    message
        .chars()
        .map(|char| char.to_uppercase().to_string())
        .map(|letter| dictionary.get(letter.as_str()))
        .map(|option| option.unwrap_or(&UNKNOWN_CHARACTER).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        std::iter::Iterator::collect(IntoIterator::into_iter([$(($key, $value),)*]))
    };
}

fn _morse_dictionary() -> HashMap<&'static str, &'static str> {
    map! {
        "A" => ".-",      "B" => "-...",    "C" => "-.-.",
        "D" => "-..",     "E" => ".",       "F" => "..-.",
        "G" => "--.",     "H" => "....",    "I" => "..",
        "J" => ".---",    "K" => "-.-",     "L" => ".-..",
        "M" => "--",      "N" => "-.",      "O" => "---",
        "P" => ".--.",    "Q" => "--.-",    "R" => ".-.",
        "S" => "...",     "T" => "-",       "U" => "..-",
        "V" => "...-",    "W" => ".--",     "X" => "-..-",
        "Y" => "-.--",    "Z" => "--..",

        "1" => ".----",   "2" => "..---",   "3" => "...--",
        "4" => "....-",   "5" => ".....",   "6" => "-....",
        "7" => "--...",   "8" => "---..",   "9" => "----.",
        "0" => "-----",

        "&" => ".-...",   "@" => ".--.-.",  ":" => "---...",
        "," => "--..--",  "." => ".-.-.-",  "'" => ".----.",
        "\"" => ".-..-.", "?" => "..--..",  "/" => "-..-.",
        "=" => "-...-",   "+" => ".-.-.",   "-" => "-....-",
        "(" => "-.--.",   ")" => "-.--.-",  " " => "/",
        "!" => "-.-.--",
    }
}

fn _morse_to_alphanumeric_dictionary() -> HashMap<&'static str, &'static str> {
    map! {
        ".-"   =>  "A",      "-..." => "B",    "-.-." => "C",
        "-.."  =>  "D",      "."    => "E",       "..-." => "F",
        "--."  =>  "G",      "...." => "H",    ".." => "I",
        ".---" =>  "J",     "-.-" => "K",     ".-.." => "L",
        "--"   =>  "M",       "-." => "N",      "---" => "O",
        ".--." =>  "P",     "--.-" => "Q",    ".-." => "R",
        "..."  =>  "S",      "-" => "T",       "..-" => "U",
        "...-" =>  "V",     ".--" => "W",     "-..-" => "X",
        "-.--" =>  "Y",     "--.." => "Z",

        ".----" => "1",    "..---" => "2",   "...--" => "3",
        "....-" => "4",    "....." => "5",   "-...." => "6",
        "--..." => "7",    "---.." => "8",   "----." => "9",
        "-----" => "0",

        ".-..." => "&",    ".--.-." => "@",  "---..." => ":",
        "--..--" => ",",   ".-.-.-" => ".",  ".----." => "'",
        ".-..-." => "\"",  "..--.." => "?",  "-..-." => "/",
        "-...-" => "=",   ".-.-." => "+",   "-....-" => "-",
        "-.--." => "(",   "-.--.-" => ")",  "/" => " ",
        "-.-.--" => "!",  " " => " ",       "" => ""
    }
}

fn _check_part(string: &str) -> bool {
    for c in string.chars() {
        match c {
            '.' | '-' | ' ' => (),
            _ => return false,
        }
    }
    true
}

fn _check_all_parts(string: &str) -> bool {
    string.split('/').all(_check_part)
}

fn _decode_token(string: &str) -> String {
    _morse_to_alphanumeric_dictionary()
        .get(string)
        .unwrap_or(&_UNKNOWN_MORSE_CHARACTER)
        .to_string()
}

fn _decode_part(string: &str) -> String {
    string
        .split(' ')
        .map(_decode_token)
        .collect::<Vec<String>>()
        .join("")
}

pub fn decode(string: &str) -> Result<String, io::Error> {
    if !_check_all_parts(string) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid morse code",
        ));
    }

    let mut partitions: Vec<String> = vec![];

    for part in string.split('/') {
        partitions.push(_decode_part(part));
    }

    Ok(partitions.join(" "))
}