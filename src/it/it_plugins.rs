use alloc::vec;
use alloc::vec::Vec;
use serde::Deserialize;

const MAX_MIXPLUGINS: usize = 64;

#[derive(Deserialize, Debug, Clone)]
#[repr(C)]
pub struct SndMixPluginInfo {
    pub id1: u32,
    pub id2: u32,
    pub input_routing: u32,
    pub output_routing: u32,
    pub reserved: [u32; 4],
}

#[derive(Deserialize, Debug, Clone)]
#[repr(C)]
pub struct MixPlugin {
    pub info: SndMixPluginInfo,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct Plugins {
    channel_settings: Vec<u32>,
    mix: Vec<MixPlugin>,
}

impl Plugins {
    pub fn new() -> Self {
        Self {
            channel_settings: vec![0; 64],
            mix: vec![
                MixPlugin {
                    info: SndMixPluginInfo {
                        id1: 0,
                        id2: 0,
                        input_routing: 0,
                        output_routing: 0,
                        reserved: [0; 4],
                    },
                    data: None,
                };
                MAX_MIXPLUGINS
            ],
        }
    }

    pub fn load(source: &[u8]) -> (Self, usize) {
        let mut data = source;
        let mut plugins = Self::new();

        while data.len() >= 8 {
            let plugin_id = u32::from_le_bytes(data[0..4].try_into().unwrap());
            let plugin_size = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;

            if plugin_size > data.len() {
                break;
            }

            if plugin_id == u32::from_le_bytes(*b"CHFX") {
                data = &data[8..];
                for ch in 0..64 {
                    if ch * 4 < plugin_size {
                        plugins.channel_settings[ch] =
                            u32::from_le_bytes(data[ch * 4..(ch + 1) * 4].try_into().unwrap());
                    }
                }
                data = &data[plugin_size..];
            } else if data[0] == b'F' && data[1] == b'X' && data[2] >= b'0' && data[3] >= b'0' {
                let plugin_index = ((data[2] - b'0') * 10 + (data[3] - b'0')) as usize;
                data = &data[8..];
                if plugin_index >= MAX_MIXPLUGINS {
                    break;
                }
                let info: (SndMixPluginInfo, usize) =
                    bincode::serde::decode_from_slice::<SndMixPluginInfo, _>(
                        data,
                        bincode::config::legacy(),
                    )
                    .unwrap();
                plugins.mix[plugin_index].info = info.0;
                data = &data[info.1..];

                let extra_size = plugin_size - info.1;
                if extra_size != 0 {
                    let d = &data[0..extra_size].to_vec();
                    plugins.mix[plugin_index].data = Some(d.clone());
                }
                data = &data[extra_size..];
            } else {
                break;
            }
        }
        (plugins, source.len() - data.len())
    }
}
