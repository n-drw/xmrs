use serde::Deserialize;
use serde_big_array::BigArray;

#[derive(Deserialize, Debug)]
#[repr(C)]
/// IT instrument header (pre-2.0).
struct ItInstrumentHeaderPre2 {
    /// Identifier ("IMPI").
    id: [u8; 4],

    /// DOS filename (12 bytes).
    filename: [u8; 12],

    /// Reserved for future use.
    reserved1: u8,

    /// Configuration flags (8 bits).
    /// - Bit 1: Use volume envelope if on.
    /// - Bit 2: Use loop envelope if on.
    /// - Bit 3: Use sustain loop envelope if on.
    flags: u8,

    /// Number of loop start node of volume envelope.
    volume_loop_start: u8,

    /// Number of loop end node of volume envelope.
    volume_loop_end: u8,

    /// Number of sustain loop start node of envelope.
    sustain_loop_start: u8,

    /// Number of sustain loop end node of envelope.
    sustain_loop_end: u8,

    /// Reserved for future use.
    reserved2: u16,

    /// Fadeout value (0-64, but counted by 512).
    fadeout: u16,

    /// New Note Action
    /// - 0: Note cut.
    /// - 1: Continue.
    /// - 2: Note off.
    /// - 3: Note fade.
    nna: u8,

    /// Disable Note Channel (DNC).
    /// - 0: Disable channel.
    /// - 1: Enable channel.
    dnc: u8,

    /// Tracker version (only used in instrument files).
    tracker_version: u16,

    /// Number of samples (only used in instrument files).
    number_of_samples: u8,

    /// Reserved for future use.
    reserved3: u8,

    /// Instrument Name
    instrument_name: [u8; 26],

    /// Reserved for future use.
    reserved4: [u8; 6],

    /// Note-to-sample mapping table
    /// - .0: Note.
    /// - .1: Sample.
    #[serde(with = "BigArray")]
    note_sample_keyboard_table: [(u8, u8); 120],
}

// Volume only
#[derive(Deserialize, Debug)]
#[repr(C)]
struct ItEnvelopePre2 {
    /// 0-64, 0xff=end of envelope
    #[serde(with = "BigArray")]
    envelope: [u8; 200],

    /// .0 = tick, .1 = magnitude
    node_points: [(u8, u8); 25],
}

// --------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
#[repr(C)]
struct ItInstrumentHeaderPost2 {
    /// Instrument identifier - must be "IMPI"
    id: [u8; 4],

    /// DOS filename
    dos_filename: [u8; 12],

    /// Reserved
    reserved1: u8,

    /// Action to take when a new note is played
    /// 0: Cut the note
    /// 1: Continue the note
    /// 2: Stop the note
    /// 3: Fade out the note
    new_note_action: u8,

    /// Duplicate check type
    /// 0: Off
    /// 1: Note
    /// 2: Sample
    /// 3: Instrument
    duplicate_check_type: u8,

    /// Action to take when a duplicate is found
    /// 0: Cut the note
    /// 1: Stop the note
    /// 2: Fade out the note
    duplicate_check_action: u8,

    /// Fade-out time (0-128, but the actual value is 1024 times larger)
    fadeout: i16,

    /// Pitch and pan separation (-32 to 32)
    pitch_pan_separation: i8,

    /// Center note for panning (0-119)
    pitch_pan_center: u8,

    /// Global volume (0-128)
    global_volume: u8,

    /// Default pan (0-64, bit 128 to ignore)
    default_pan: u8,

    /// Random volume variation (0-100)
    random_volume_variation: u8,

    /// Random pan variation (0-100)
    random_pan_variation: u8,

    /// Tracker version used to save the instrument (only used in instrument files)
    tracker_version: u16,

    /// Number of samples used by this instrument (only used in instrument files)
    num_samples: u8,

    /// Reserved
    reserved2: u8,

    /// Instrument name
    instrument_name: [u8; 26],

    /// Initial filter cutoff frequency (0-127)
    /// The formula used is 110*2^(0.25+ce/fe), where ce is the cutoff frequency * (256 + 256) and fe is 24*512 or 20*512 if using OpenMPT's extended filter range.
    initial_filter_cutoff: i8,

    /// Initial filter resonance (0-127)
    /// The formula used is 10^((-resonance*24.0)/(128.0f*20.0f)), but it's generally better to use a precalculated table.
    initial_filter_resonance: i8,

    /// MIDI channel (0-16)
    midi_channel: i8,

    /// MIDI program (1-128)
    midi_program: u8,

    /// MIDI bank (0-16384)
    midi_bank: u16,

    /// Note-sample-keyboard table (120 entries)
    /// .0: Note
    /// .1: Sample
    #[serde(with = "BigArray")]
    note_sample_keyboard_table: [(u8, u8); 120],
}

#[derive(Deserialize, Debug, Default)]
#[repr(C)]
struct ItEnvelopePost2 {
    /// Envelope flags
    /// - Bit 0: Enable/disable envelope
    /// - Bit 1: Enable/disable loop
    /// - Bit 2: Enable/disable sustain loop
    /// - Bit 3: Reserved (used as envelope carry in OpenMPT)
    /// - Bits 4-6: Reserved
    /// - Bit 7: Use pitch envelope as filter (only applies to pitch envelope)
    flags: u8,

    /// Number of valid nodes in the file
    node_count: u8,

    /// Starting node of the loop
    loop_start: u8,

    /// Ending node of the loop
    loop_end: u8,

    /// Starting node of the sustain loop
    sustain_loop_start: u8,

    /// Ending node of the sustain loop
    sustain_loop_end: u8,

    /// Node points table
    /// - .0: Node value (0-64 for volume and filter, -32 to 32 for pan and pitch)
    /// - .1: Node position in ticks (0-9999)
    node_points: [(u8, u16); 25],
    // trailing_bytes: [u8; 7], // 7 bytes if version 2.0 to 2.14, 4 bytes if 2.14p1 or above
}

// --------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
struct ItInstrumentPre2 {
    instr: ItInstrumentHeaderPre2,
    volume_envelope: ItEnvelopePre2,
}

#[derive(Deserialize, Debug)]
struct ItInstrumentPost2 {
    instr: ItInstrumentHeaderPost2,
    volume_envelope: ItEnvelopePost2,
    panning_envelope: ItEnvelopePost2,
    pitch_envelope: ItEnvelopePost2,
}

#[derive(Deserialize, Debug)]
pub enum ItInstrument {
    Pre2(ItInstrumentPre2),
    Post2(ItInstrumentPost2),
}

impl ItInstrument {
    pub fn load_post2(source: &[u8]) -> Self {
        let mut data = source;

        let instr_h = bincode::serde::decode_from_slice::<ItInstrumentHeaderPre2, _>(
            data,
            bincode::config::legacy(),
        )
        .unwrap();
        data = &data[instr_h.1..];
        let vol = bincode::serde::decode_from_slice::<ItEnvelopePre2, _>(
            data,
            bincode::config::legacy(),
        )
        .unwrap();
        let instr = ItInstrumentPre2 {
            instr: instr_h.0,
            volume_envelope: vol.0,
        };
        return ItInstrument::Pre2(instr);
    }

    pub fn load_pre2(source: &[u8]) -> Self {
        let mut data = source;

        let instr_h = bincode::serde::decode_from_slice::<ItInstrumentHeaderPost2, _>(
            data,
            bincode::config::legacy(),
        )
        .unwrap();
        data = &data[instr_h.1..];
        let vol = bincode::serde::decode_from_slice::<ItEnvelopePost2, _>(
            data,
            bincode::config::legacy(),
        )
        .unwrap();
        data = &data[1 + vol.1..];
        let pan = bincode::serde::decode_from_slice::<ItEnvelopePost2, _>(
            data,
            bincode::config::legacy(),
        )
        .unwrap();
        data = &data[1 + pan.1..];
        let pitch = bincode::serde::decode_from_slice::<ItEnvelopePost2, _>(
            data,
            bincode::config::legacy(),
        )
        .unwrap();
        let instr = ItInstrumentPost2 {
            instr: instr_h.0,
            volume_envelope: vol.0,
            panning_envelope: pan.0,
            pitch_envelope: pitch.0,
        };
        return ItInstrument::Post2(instr);
    }


}
