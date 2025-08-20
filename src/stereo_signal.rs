use std::{fs::File, sync::Arc};

use claxon::FlacReader;

use crate::{
    audio_file::types::{BitDepth, Frequency, Signal},
    constants::{MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, MAX_8_BIT},
    sonicprobe_error::SonicProbeError,
};

pub struct StereoSignal {
    pub left: Signal,
    pub right: Signal,
    pub samples_per_channel: usize,
    pub sample_rate: Frequency,
    pub depth: BitDepth,
}

impl StereoSignal {
    pub fn from_flac(stream: FlacReader<File>) -> Result<Self, SonicProbeError> {
        let infos = stream.streaminfo();

        if infos.channels != 2 {
            return Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: "Currently only stereo signal is supported".to_owned(),
            });
        }

        let sample_rate = Frequency::new(infos.sample_rate)?;
        let depth = BitDepth::new(infos.bits_per_sample)?;
        
        let (left, right) = read_audio_signal(stream, depth)?;
        let samples_per_channel: usize = left.len();
        
        Ok(Self {
            left,
            right,
            samples_per_channel,
            sample_rate,
            depth,
        })
    }
}

fn read_audio_signal(
    mut stream: FlacReader<File>,
    depth: BitDepth,
) -> Result<(Signal, Signal), SonicProbeError> {
    let mut left: Vec<f64> = Vec::new();
    let mut right: Vec<f64> = Vec::new();
    let multiplier = match depth {
        BitDepth::Legacy => MAX_8_BIT,
        BitDepth::CdStandard => MAX_16_BIT,
        BitDepth::Professional => MAX_24_BIT,
        BitDepth::StudioMaster => MAX_32_BIT,
    };


    for (index, sample) in stream.samples().enumerate() {
        if index % 2 == 1 {
            right.push(f64::from(sample?) / multiplier);
        } else {
            left.push(f64::from(sample?) / multiplier);
        }
    }

    Ok((Arc::from(right), Arc::from(left)))
}
