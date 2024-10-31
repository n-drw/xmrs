use serde::Deserialize;
use serde_big_array::BigArray;

/// IT file header.
#[derive(Deserialize, Debug)]
#[repr(C)]
pub struct ItHeader {
    /// Identifier ("IMPM").
    id: [u8; 4],

    /// Song name (26 bytes, including nulls).
    pub song_name: [u8; 26],

    /// Pattern information
    /// - The first byte represents the number of rows per beat.
    /// - The second byte represents the number of rows per measure.
    pub pattern_highlight: [u8; 2],

    /// Number of sequenced patterns in the song.
    pub order_number: u16,

    /// Number of instruments in the song.
    pub instrument_number: u16,

    /// Number of samples in the song.
    pub sample_number: u16,

    /// Number of patterns in the song.
    pub pattern_number: u16,

    /// ID of the tracker that made this file (1.0 to 2.17, encoded as hexadecimal).
    pub created_with_tracker: u16,

    /// Version of the tracker compatible with this file (1.0 to 2.17, encoded as hexadecimal).
    pub compatible_with_tracker: u16,

    /// Configuration flags (16 bits).
    /// - Bit 0: Stereo if on, mono if off.
    /// - Bit 1: Mixing (obsolete since version 1.04).
    /// - Bit 2: Use instruments if on, use samples if off.
    /// - Bit 3: Use linear slides if on, use Amiga slides if off.
    /// - Bit 4: Use old effects if on, use IT effects if off.
    /// - Bit 5: Link G effect with E and F memory if on.
    /// - Bit 6: MIDI pitch controlled if on.
    /// - Bit 7: Request Embedded MIDI Macros if on.
    /// - Bits 8-15 are reserved.
    pub flags: u16,

    /// Special configuration flags (16 bits).
    /// - Bit 0: Song message attached if on.
    /// - Bit 1: Edit history embedded if on (Some versions of Schism tracker set this off even if there is embedded history).
    /// - Bit 2: Highlight embedded if on.
    /// - Bit 3: Embedded MIDI Macro.
    /// - Bits 4-15 are reserved.
    pub special_flags: u16,

    /// Global volume of the song (0-128).
    pub global_volume: u8,

    /// Mixing volume of the song (0-128).
    pub mix_volume: u8,

    /// Starting tick speed of the song.
    pub initial_speed: u8,

    /// Starting beats per minute of the song.
    pub initial_tempo: u8,

    /// Panning separation between channels (0-128).
    pub pan_separation: u8,

    /// Pitch wheel depth for MIDI controllers.
    pub pitch_wheel_depth: u8,

    /// Length of the attached message.
    pub message_length: u16,

    /// Offset of the message in the file.
    pub message_offset: u32,

    /// Reserved field ("OMPT" for interpreted Mod Plug files).
    pub reserved: [u8; 4],

    /// Initial pan of the channels
    /// Each byte is a pan value (examples: 0 is left pan, 32 is center pan and 64 is right pan).
    #[serde(with = "BigArray")]
    pub initial_channel_pan: [u8; 64],

    /// Initial volume of the channels
    /// Each byte is a channel volume (0-64).
    #[serde(with = "BigArray")]
    pub initial_channel_volume: [u8; 64],
}

impl ItHeader {
    /// Mod Plug file?
    pub fn is_ompt(&self) -> bool {
        self.reserved[0] == b'O'
            && self.reserved[1] == b'M'
            && self.reserved[2] == b'P'
            && self.reserved[3] == b'T'
    }

    pub fn is_post20(&self) -> bool {
        (self.created_with_tracker >> 8) >= 2
    }
}
