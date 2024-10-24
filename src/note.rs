use core::cmp::PartialEq;
use core::fmt::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// 10 octaves with notes
#[derive(
    Default,
    Serialize,
    Deserialize,
    TryFromPrimitive,
    IntoPrimitive,
    Copy,
    Clone,
    Eq,
    Hash,
    PartialEq,
)]
#[repr(u8)]
pub enum Note {
    /// Octave 0
    C0 = 0,
    Cs0 = 1,
    D0 = 2,
    Ds0 = 3,
    E0 = 4,
    F0 = 5,
    Fs0 = 6,
    G0 = 7,
    Gs0 = 8,
    A0 = 9,
    As0 = 10,
    B0 = 11,
    /// Octave 1
    C1 = 12,
    Cs1 = 13,
    D1 = 14,
    Ds1 = 15,
    E1 = 16,
    F1 = 17,
    Fs1 = 18,
    G1 = 19,
    Gs1 = 20,
    A1 = 21,
    As1 = 22,
    B1 = 23,
    /// Octave 2
    C2 = 24,
    Cs2 = 25,
    D2 = 26,
    Ds2 = 27,
    E2 = 28,
    F2 = 29,
    Fs2 = 30,
    G2 = 31,
    Gs2 = 32,
    A2 = 33,
    As2 = 34,
    B2 = 35,
    /// Octave 3
    C3 = 36,
    Cs3 = 37,
    D3 = 38,
    Ds3 = 39,
    E3 = 40,
    F3 = 41,
    Fs3 = 42,
    G3 = 43,
    Gs3 = 44,
    A3 = 45,
    As3 = 46,
    B3 = 47,
    /// Octave 4
    C4 = 48,
    Cs4 = 49,
    D4 = 50,
    Ds4 = 51,
    E4 = 52,
    F4 = 53,
    Fs4 = 54,
    G4 = 55,
    Gs4 = 56,
    A4 = 57,
    As4 = 58,
    B4 = 59,
    /// Octave 5
    C5 = 60,
    Cs5 = 61,
    D5 = 62,
    Ds5 = 63,
    E5 = 64,
    F5 = 65,
    Fs5 = 66,
    G5 = 67,
    Gs5 = 68,
    A5 = 69,
    As5 = 70,
    B5 = 71,
    /// Octave 6
    C6 = 72,
    Cs6 = 73,
    D6 = 74,
    Ds6 = 75,
    E6 = 76,
    F6 = 77,
    Fs6 = 78,
    G6 = 79,
    Gs6 = 80,
    A6 = 81,
    As6 = 82,
    B6 = 83,
    /// Octave 7
    C7 = 84,
    Cs7 = 85,
    D7 = 86,
    Ds7 = 87,
    E7 = 88,
    F7 = 89,
    Fs7 = 90,
    G7 = 91,
    Gs7 = 92,
    A7 = 93,
    As7 = 94,
    B7 = 95,
    /// Octave 8
    C8 = 96,
    Cs8 = 97,
    D8 = 98,
    Ds8 = 99,
    E8 = 100,
    F8 = 101,
    Fs8 = 102,
    G8 = 103,
    Gs8 = 104,
    A8 = 105,
    As8 = 106,
    B8 = 107,
    /// Octave 9
    C9 = 108,
    Cs9 = 109,
    D9 = 110,
    Ds9 = 111,
    E9 = 112,
    F9 = 113,
    Fs9 = 114,
    G9 = 115,
    Gs9 = 116,
    A9 = 117,
    As9 = 118,
    B9 = 119,

    #[default]
    None = 253,
    /// Cut Note
    Cut = 254, // Like IT, not like S3M.
    /// Stop note, or Fadout
    KeyOff = 255, // Like IT, not like XM (97)
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let text = match self {
            // Octave 0
            Note::C0 => "C-0",
            Note::Cs0 => "C#0",
            Note::D0 => "D-0",
            Note::Ds0 => "D#0",
            Note::E0 => "E-0",
            Note::F0 => "F-0",
            Note::Fs0 => "F#0",
            Note::G0 => "G-0",
            Note::Gs0 => "G#0",
            Note::A0 => "A-0",
            Note::As0 => "A#0",
            Note::B0 => "B-0",
            // Octave 1
            Note::C1 => "C-1",
            Note::Cs1 => "C#1",
            Note::D1 => "D-1",
            Note::Ds1 => "D#1",
            Note::E1 => "E-1",
            Note::F1 => "F-1",
            Note::Fs1 => "F#1",
            Note::G1 => "G-1",
            Note::Gs1 => "G#1",
            Note::A1 => "A-1",
            Note::As1 => "A#1",
            Note::B1 => "B-1",
            // Octave 2
            Note::C2 => "C-2",
            Note::Cs2 => "C#2",
            Note::D2 => "D-2",
            Note::Ds2 => "D#2",
            Note::E2 => "E-2",
            Note::F2 => "F-2",
            Note::Fs2 => "F#2",
            Note::G2 => "G-2",
            Note::Gs2 => "G#2",
            Note::A2 => "A-2",
            Note::As2 => "A#2",
            Note::B2 => "B-2",
            // Octave 3
            Note::C3 => "C-3",
            Note::Cs3 => "C#3",
            Note::D3 => "D-3",
            Note::Ds3 => "D#3",
            Note::E3 => "E-3",
            Note::F3 => "F-3",
            Note::Fs3 => "F#3",
            Note::G3 => "G-3",
            Note::Gs3 => "G#3",
            Note::A3 => "A-3",
            Note::As3 => "A#3",
            Note::B3 => "B-3",
            // Octave 4
            Note::C4 => "C-4",
            Note::Cs4 => "C#4",
            Note::D4 => "D-4",
            Note::Ds4 => "D#4",
            Note::E4 => "E-4",
            Note::F4 => "F-4",
            Note::Fs4 => "F#4",
            Note::G4 => "G-4",
            Note::Gs4 => "G#4",
            Note::A4 => "A-4",
            Note::As4 => "A#4",
            Note::B4 => "B-4",
            // Octave 5
            Note::C5 => "C-5",
            Note::Cs5 => "C#5",
            Note::D5 => "D-5",
            Note::Ds5 => "D#5",
            Note::E5 => "E-5",
            Note::F5 => "F-5",
            Note::Fs5 => "F#5",
            Note::G5 => "G-5",
            Note::Gs5 => "G#5",
            Note::A5 => "A-5",
            Note::As5 => "A#5",
            Note::B5 => "B-5",
            // Octave 6
            Note::C6 => "C-6",
            Note::Cs6 => "C#6",
            Note::D6 => "D-6",
            Note::Ds6 => "D#6",
            Note::E6 => "E-6",
            Note::F6 => "F-6",
            Note::Fs6 => "F#6",
            Note::G6 => "G-6",
            Note::Gs6 => "G#6",
            Note::A6 => "A-6",
            Note::As6 => "A#6",
            Note::B6 => "B-6",
            // Octave 7
            Note::C7 => "C-7",
            Note::Cs7 => "C#7",
            Note::D7 => "D-7",
            Note::Ds7 => "D#7",
            Note::E7 => "E-7",
            Note::F7 => "F-7",
            Note::Fs7 => "F#7",
            Note::G7 => "G-7",
            Note::Gs7 => "G#7",
            Note::A7 => "A-7",
            Note::As7 => "A#7",
            Note::B7 => "B-7",
            // Octave 8
            Note::C8 => "C-8",
            Note::Cs8 => "C#8",
            Note::D8 => "D-8",
            Note::Ds8 => "D#8",
            Note::E8 => "E-8",
            Note::F8 => "F-8",
            Note::Fs8 => "F#8",
            Note::G8 => "G-8",
            Note::Gs8 => "G#8",
            Note::A8 => "A-8",
            Note::As8 => "A#8",
            Note::B8 => "B-8",
            // Octave 9
            Note::C9 => "C-9",
            Note::Cs9 => "C#9",
            Note::D9 => "D-9",
            Note::Ds9 => "D#9",
            Note::E9 => "E-9",
            Note::F9 => "F-9",
            Note::Fs9 => "F#9",
            Note::G9 => "G-9",
            Note::Gs9 => "G#9",
            Note::A9 => "A-9",
            Note::As9 => "A#9",
            Note::B9 => "B-9",

            Note::None => "---",
            Note::KeyOff => "===",
            Note::Cut => "^^^"
        };
        write!(f, "{}", text)
    }
}

impl Note {
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Self::None
    }

    #[inline(always)]
    pub fn is_keyoff(&self) -> bool {
        *self == Self::KeyOff
    }

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.value() < 120
    }

    #[inline(always)]
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
