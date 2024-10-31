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
pub enum Pitch {
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

impl Debug for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let text = match self {
            // Octave 0
            Pitch::C0 => "C-0",
            Pitch::Cs0 => "C#0",
            Pitch::D0 => "D-0",
            Pitch::Ds0 => "D#0",
            Pitch::E0 => "E-0",
            Pitch::F0 => "F-0",
            Pitch::Fs0 => "F#0",
            Pitch::G0 => "G-0",
            Pitch::Gs0 => "G#0",
            Pitch::A0 => "A-0",
            Pitch::As0 => "A#0",
            Pitch::B0 => "B-0",
            // Octave 1
            Pitch::C1 => "C-1",
            Pitch::Cs1 => "C#1",
            Pitch::D1 => "D-1",
            Pitch::Ds1 => "D#1",
            Pitch::E1 => "E-1",
            Pitch::F1 => "F-1",
            Pitch::Fs1 => "F#1",
            Pitch::G1 => "G-1",
            Pitch::Gs1 => "G#1",
            Pitch::A1 => "A-1",
            Pitch::As1 => "A#1",
            Pitch::B1 => "B-1",
            // Octave 2
            Pitch::C2 => "C-2",
            Pitch::Cs2 => "C#2",
            Pitch::D2 => "D-2",
            Pitch::Ds2 => "D#2",
            Pitch::E2 => "E-2",
            Pitch::F2 => "F-2",
            Pitch::Fs2 => "F#2",
            Pitch::G2 => "G-2",
            Pitch::Gs2 => "G#2",
            Pitch::A2 => "A-2",
            Pitch::As2 => "A#2",
            Pitch::B2 => "B-2",
            // Octave 3
            Pitch::C3 => "C-3",
            Pitch::Cs3 => "C#3",
            Pitch::D3 => "D-3",
            Pitch::Ds3 => "D#3",
            Pitch::E3 => "E-3",
            Pitch::F3 => "F-3",
            Pitch::Fs3 => "F#3",
            Pitch::G3 => "G-3",
            Pitch::Gs3 => "G#3",
            Pitch::A3 => "A-3",
            Pitch::As3 => "A#3",
            Pitch::B3 => "B-3",
            // Octave 4
            Pitch::C4 => "C-4",
            Pitch::Cs4 => "C#4",
            Pitch::D4 => "D-4",
            Pitch::Ds4 => "D#4",
            Pitch::E4 => "E-4",
            Pitch::F4 => "F-4",
            Pitch::Fs4 => "F#4",
            Pitch::G4 => "G-4",
            Pitch::Gs4 => "G#4",
            Pitch::A4 => "A-4",
            Pitch::As4 => "A#4",
            Pitch::B4 => "B-4",
            // Octave 5
            Pitch::C5 => "C-5",
            Pitch::Cs5 => "C#5",
            Pitch::D5 => "D-5",
            Pitch::Ds5 => "D#5",
            Pitch::E5 => "E-5",
            Pitch::F5 => "F-5",
            Pitch::Fs5 => "F#5",
            Pitch::G5 => "G-5",
            Pitch::Gs5 => "G#5",
            Pitch::A5 => "A-5",
            Pitch::As5 => "A#5",
            Pitch::B5 => "B-5",
            // Octave 6
            Pitch::C6 => "C-6",
            Pitch::Cs6 => "C#6",
            Pitch::D6 => "D-6",
            Pitch::Ds6 => "D#6",
            Pitch::E6 => "E-6",
            Pitch::F6 => "F-6",
            Pitch::Fs6 => "F#6",
            Pitch::G6 => "G-6",
            Pitch::Gs6 => "G#6",
            Pitch::A6 => "A-6",
            Pitch::As6 => "A#6",
            Pitch::B6 => "B-6",
            // Octave 7
            Pitch::C7 => "C-7",
            Pitch::Cs7 => "C#7",
            Pitch::D7 => "D-7",
            Pitch::Ds7 => "D#7",
            Pitch::E7 => "E-7",
            Pitch::F7 => "F-7",
            Pitch::Fs7 => "F#7",
            Pitch::G7 => "G-7",
            Pitch::Gs7 => "G#7",
            Pitch::A7 => "A-7",
            Pitch::As7 => "A#7",
            Pitch::B7 => "B-7",
            // Octave 8
            Pitch::C8 => "C-8",
            Pitch::Cs8 => "C#8",
            Pitch::D8 => "D-8",
            Pitch::Ds8 => "D#8",
            Pitch::E8 => "E-8",
            Pitch::F8 => "F-8",
            Pitch::Fs8 => "F#8",
            Pitch::G8 => "G-8",
            Pitch::Gs8 => "G#8",
            Pitch::A8 => "A-8",
            Pitch::As8 => "A#8",
            Pitch::B8 => "B-8",
            // Octave 9
            Pitch::C9 => "C-9",
            Pitch::Cs9 => "C#9",
            Pitch::D9 => "D-9",
            Pitch::Ds9 => "D#9",
            Pitch::E9 => "E-9",
            Pitch::F9 => "F-9",
            Pitch::Fs9 => "F#9",
            Pitch::G9 => "G-9",
            Pitch::Gs9 => "G#9",
            Pitch::A9 => "A-9",
            Pitch::As9 => "A#9",
            Pitch::B9 => "B-9",

            Pitch::None => "---",
            Pitch::KeyOff => "===",
            Pitch::Cut => "^^^",
        };
        write!(f, "{}", text)
    }
}

impl Pitch {
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
