// Table for converting usize key to &str key
pub const STATES: &'static [&str] = &[
    "C",
    "Db",
    "D",
    "Eb",
    "E",
    "F",
    "F#",
    "G",
    "Ab",
    "A",
    "Bb",
    "B",
    "c",
    "c#",
    "d",
    "eb",
    "e",
    "f",
    "f#",
    "g",
    "ab",
    "a",
    "bb",
    "b"
];

pub const START_STATE: [f64; 24] = [1.0/24.0; 24];