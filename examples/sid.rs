#![forbid(unsafe_code)]

use bincode::error::EncodeError;
use std::fs::File;
use std::io::prelude::*;
use xmrs::module::Module;
use xmrs::import::sid::sid_module::SidModule;

fn main() -> Result<(), EncodeError> {
    println!("--===~ XmRs SID Module Info Example ~===--");
    println!("(c) 2024 Sébastien Béchet\n");

    //TODO: SOUNDFX
    println!("Warning: it's just a game to extract some data. Don't expect anything beautiful.");

    // SidModule::get_sid_commando();
    let sid = SidModule::get_sid_crazy_comets();
    let sid = SidModule::get_sid_monty_on_the_run();
    let sid = SidModule::get_sid_last_v8();
    let sid = SidModule::get_sid_thing_on_a_spring();
    let sid = SidModule::get_sid_zoid();
    let sid = SidModule::get_sid_ace_2();
    // SidModule::get_sid_delta(); // FIXME: (v30) src/sid/pattern_helper.rs:186:42:
    let sid = SidModule::get_sid_human_race();
    // SidModule::get_sid_international_karate(); // Data is really strange. Maybe i have a bug somewhere.
    let sid = SidModule::get_sid_lightforce();
    let sid = SidModule::get_sid_sanxion_song_1();
    let sid = SidModule::get_sid_sanxion_song_2();
    let sid = SidModule::get_sid_spellbound();

    // println!("{:?}", sid);


    Ok(())
}
