use std::{fs::File, sync::Arc};

use flac::{ReadStream, Stream};

use crate::{
    audio_file::types::{BitPrecision, Signal},
    constants::{MAX_8_BIT, MAX_16_BIT, MAX_24_BIT, MAX_32_BIT},
};

const VALID_SAMPLE_RATES: [u32; 6] = [44100, 48000, 88200, 96000, 176_400, 192_000];

pub struct StereoSignal {
    pub interleaved: Signal,
    pub left: Signal,
    pub right: Signal,
    pub samples_per_channel: u64,
    pub sample_rate: u32,
    pub depth: BitPrecision,
}

impl StereoSignal {
    pub fn from_flac(stream: Stream<ReadStream<File>>) -> Result<Self, String> {
        let infos = stream.info();

        if infos.channels != 2 {
            return Err("Currently only stereo signal is supporated".to_owned());
        }

        if !VALID_SAMPLE_RATES.contains(&infos.sample_rate) {
            return Err(format!(
                "Currently only signal in {VALID_SAMPLE_RATES:?} sample rate are supporated"
            ));
        }

        let sample_rate = infos.sample_rate;
        let depth = infos.bits_per_sample;
        let samples_per_channel = infos.total_samples;

        let interleaved = read_audio_signal(stream, depth)?;

        let (left, right) = deinterleave(&interleaved)?;

        Ok(Self {
            interleaved,
            left,
            right,
            samples_per_channel,
            sample_rate,
            depth,
        })
    }
}

fn deinterleave(interleaved: &Signal) -> Result<(Signal, Signal), String> {
    let channel_size = interleaved.len() / 2;
    let mut left: Vec<f64> = Vec::with_capacity(channel_size);
    let mut right: Vec<f64> = Vec::with_capacity(channel_size);

    for pair in interleaved.chunks_exact(2) {
        left.push(*pair.first().ok_or("error: mismatch in channels size")?);
        right.push(*pair.last().ok_or("error: mismatch in channels size")?);
    }

    Ok((Arc::from(left), Arc::from(right)))
}

fn read_audio_signal(
    mut stream: Stream<ReadStream<File>>,
    depth: BitPrecision,
) -> Result<Signal, String> {
    match depth {
        8 => Ok(stream
            .iter::<i8>()
            .map(std::convert::Into::into)
            .map(|s: f64| s / MAX_8_BIT)
            .collect::<Signal>()),
        16 => Ok(stream
            .iter::<i16>()
            .map(std::convert::Into::into)
            .map(|s: f64| s / MAX_16_BIT)
            .collect::<Signal>()),
        24 => Ok(stream
            .iter::<i32>()
            .map(|s| (s >> 8).into())
            .map(|s: f64| s / MAX_24_BIT)
            .collect::<Signal>()),
        32 => Ok(stream
            .iter::<i32>()
            .map(std::convert::Into::into)
            .map(|s: f64| s / MAX_32_BIT)
            .collect::<Signal>()),
        _ => Err(format!("Unknown bit depth: {depth} bit")),
    }
}
