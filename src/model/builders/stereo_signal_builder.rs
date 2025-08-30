use std::{fs::File, sync::Arc};

use claxon::FlacReader;

use crate::model::{
    MAX_8_BIT, MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, Signal, bit_depth::BitDepth,
    frequency::Frequency, sonicprobe_error::SonicProbeError, stereo_signal::StereoSignal,
};

pub fn stereo_signal_from_flac(stream: FlacReader<File>) -> Result<StereoSignal, SonicProbeError> {
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

    Ok(StereoSignal {
        left,
        right,
        sample_rate,
        depth,
    })
}

fn read_audio_signal(
    mut stream: FlacReader<File>,
    depth: BitDepth,
) -> Result<(Signal, Signal), SonicProbeError> {
    let size: usize = match stream.streaminfo().samples {
        Some(count) => count.try_into()?,
        None => 0,
    };
    let mut left: Vec<f64> = Vec::with_capacity(size);
    let mut right: Vec<f64> = Vec::with_capacity(size);
    let multiplier = match depth {
        BitDepth::Legacy => MAX_8_BIT,
        BitDepth::CdStandard => MAX_16_BIT,
        BitDepth::Professional => MAX_24_BIT,
        BitDepth::StudioMaster => MAX_32_BIT,
    };

    for (index, sample) in stream.samples().enumerate() {
        if index % 2 == 1 {
            left.push(f64::from(sample?) / multiplier);
        } else {
            right.push(f64::from(sample?) / multiplier);
        }
    }

    Ok((Arc::from(right), Arc::from(left)))
}
