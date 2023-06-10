pub fn encode_ascii(string: &str) -> String {
    string
        .chars()
        .map(|c| match c {
            'a' | 'A' => "11",
            'b' | 'B' => "12",
            'c' | 'C' => "13",
            'd' | 'D' => "14",
            'e' | 'E' => "15",
            'f' | 'F' => "21",
            'g' | 'G' => "22",
            'h' | 'H' => "23",
            'i' | 'I' | 'j' | 'J' => "24",
            'k' | 'K' => "25",
            'l' | 'L' => "31",
            'm' | 'M' => "32",
            'n' | 'N' => "33",
            'o' | 'O' => "34",
            'p' | 'P' => "35",
            'q' | 'Q' => "41",
            'r' | 'R' => "42",
            's' | 'S' => "43",
            't' | 'T' => "44",
            'u' | 'U' => "45",
            'v' | 'V' => "51",
            'w' | 'W' => "52",
            'x' | 'X' => "53",
            'y' | 'Y' => "54",
            'z' | 'Z' => "55",
            _ => "",
        })
        .collect()
}

pub fn decode_ascii(string: &str) -> String {
    string
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .as_bytes()
        .chunks(2)
        .map(|s| match std::str::from_utf8(s) {
            Ok(v) => v.parse::<i32>().unwrap_or(0),
            Err(_) => 0,
        })
        .map(|i| match i {
            11 => 'A',
            12 => 'B',
            13 => 'C',
            14 => 'D',
            15 => 'E',
            21 => 'F',
            22 => 'G',
            23 => 'H',
            24 => 'I',
            25 => 'K',
            31 => 'L',
            32 => 'M',
            33 => 'N',
            34 => 'O',
            35 => 'P',
            41 => 'Q',
            42 => 'R',
            43 => 'S',
            44 => 'T',
            45 => 'U',
            51 => 'V',
            52 => 'W',
            53 => 'X',
            54 => 'Y',
            55 => 'Z',
            _ => ' ',
        })
        .collect::<String>()
        .replace(' ', "")
}