mod analysis;
pub mod channel;

use std::fs::File;
use std::sync::Arc;
use std::thread;

use flac::{ReadStream, Stream};

use crate::audio_file::analysis::ActualBitDepth;
use crate::audio_file::analysis::ClippingSamples;
use crate::audio_file::analysis::Peak;
use crate::audio_file::analysis::StereoCorrelation;
use crate::audio_file::channel::Channel;
use crate::constants::MAX_8_BIT;
use crate::constants::MAX_16_BIT;
use crate::constants::MAX_24_BIT;
use crate::constants::MAX_32_BIT;
use crate::dsp::upsample;

pub type Signal = Arc<[f64]>;
type Frequency = u32;
type BitPrecision = u8;
type Milliseconds = i64;

pub struct AudioFile {
    left: Channel,  // OK
    right: Channel, // OK
    /* Group next 4 */
    samples_per_channel: u64,
    sample_rate: Frequency,
    duration: Milliseconds,
    stereo_correlation: f64,
    channels: u8, // OK
    /* Group next 2 */
    depth: BitPrecision,
    true_depth: BitPrecision,
}

fn new_channel_thread(
    samples: Signal,
    sample_rate: Frequency,
    samples_per_channel: u64,
) -> std::thread::JoinHandle<Channel> {
    thread::spawn(move || Channel::from_samples(&samples, sample_rate, samples_per_channel))
}

fn new_upsample_thread(
    data: Signal,
    original_sample_rate: Frequency,
) -> std::thread::JoinHandle<(f64, usize)> {
    thread::spawn(move || {
        let signal = upsample(data, original_sample_rate);

        let peak = Peak::process(&signal);
        let clip_count = ClippingSamples::process(&signal);

        (peak, clip_count)
    })
}

impl AudioFile {
    pub fn new(data_stream: Stream<ReadStream<File>>) -> Result<Self, String> {
        let channel_count = data_stream.info().channels;
        let sample_rate = data_stream.info().sample_rate;
        let depth = data_stream.info().bits_per_sample;
        let samples_per_channel = data_stream.info().total_samples;

        let signal = read_flac_file(data_stream, depth)?;

        let (left_samples, right_samples) = split_sample_array_into_channels(&signal)?;

        let left_upsample_worker = new_upsample_thread(Arc::clone(&left_samples), sample_rate);
        let right_upsample_worker = new_upsample_thread(Arc::clone(&right_samples), sample_rate);
        let left_worker =
            new_channel_thread(Arc::clone(&left_samples), sample_rate, samples_per_channel);
        let right_worker =
            new_channel_thread(Arc::clone(&right_samples), sample_rate, samples_per_channel);

        let true_bit_depth = ActualBitDepth::process(&signal, depth);
        let stereo_correlation = StereoCorrelation::process(&left_samples, &right_samples);

        let mut left = match left_worker.join() {
            Ok(value) => value,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };
        let mut right = match right_worker.join() {
            Ok(value) => value,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };
        let (left_true_peak, left_true_clipping_samples_count) = match left_upsample_worker.join() {
            Ok(values) => values,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };
        let (right_true_peak, right_true_clipping_samples_count) =
            match right_upsample_worker.join() {
                Ok(values) => values,
                Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
            };

        let signed_sample_count: i64 = match samples_per_channel.try_into() {
            Ok(value) => value,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };

        left.true_peak = left_true_peak;
        left.true_clipping_samples_count = left_true_clipping_samples_count;
        right.true_peak = right_true_peak;
        right.true_clipping_samples_count = right_true_clipping_samples_count;

        Ok(Self {
            left,
            right,
            depth,
            channels: channel_count,
            sample_rate,
            duration: signed_sample_count / i64::from(sample_rate),
            samples_per_channel,
            stereo_correlation,
            true_depth: true_bit_depth,
        })
    }

    pub const fn left(&self) -> &Channel {
        &self.left
    }

    pub const fn right(&self) -> &Channel {
        &self.right
    }

    pub const fn channel_count(&self) -> u8 {
        self.channels
    }

    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub const fn depth(&self) -> BitPrecision {
        self.depth
    }

    pub const fn true_bit_depth(&self) -> u8 {
        self.true_depth
    }

    pub fn rms_balance(&self) -> f64 {
        self.left.rms() - self.right.rms()
    }

    pub const fn duration(&self) -> i64 {
        self.duration
    }

    pub const fn samples_count(&self) -> u64 {
        self.samples_per_channel
    }

    pub const fn stereo_correlation(&self) -> f64 {
        self.stereo_correlation
    }

    pub fn to_json_string(&self) -> String {
        let inner_tab: String = "\t".to_owned();
        let output = [
            format!(
                "{}\"channel_count\": {},\n",
                inner_tab,
                self.channel_count()
            ),
            format!("{}\"sample_rate\": {},\n", inner_tab, self.sample_rate()),
            format!("{}\"bit_depth\": {},\n", inner_tab, self.depth()),
            format!(
                "{}\"true_bit_depth\": {},\n",
                inner_tab,
                self.true_bit_depth()
            ),
            format!("{}\"duration\": {},\n", inner_tab, self.duration()),
            format!(
                "{}\"samples_count\": {},\n",
                inner_tab,
                self.samples_count()
            ),
            format!("{}\"rms_balance\": {},\n", inner_tab, self.rms_balance()),
            format!(
                "{}\"stereo_correlation\": {},\n",
                inner_tab,
                self.stereo_correlation()
            ),
            format!("{}\"left\": {},\n", inner_tab, self.left.as_json_string(1)),
            format!("{}\"right\": {}\n", inner_tab, self.right.as_json_string(1)),
        ]
        .concat();

        format!("{{\n{output}}}",)
    }
}

fn split_sample_array_into_channels(samples: &Signal) -> Result<(Signal, Signal), String> {
    let pairs: Result<Vec<(f64, f64)>, String> = samples
        .chunks_exact(2)
        .map(|pair| {
            let left_sample = pair.first().ok_or("error: mismatch in channels size")?;
            let right_sample = pair.last().ok_or("error: mismatch in channels size")?;
            Ok((*left_sample, *right_sample))
        })
        .collect();

    let (left_vec, right_vec): (Vec<f64>, Vec<f64>) = pairs?.into_iter().unzip();

    Ok((Arc::from(left_vec), Arc::from(right_vec)))
}

fn read_flac_file(mut data_stream: Stream<ReadStream<File>>, depth: BitPrecision) -> Result<Signal, String> {
    match depth {
        8 => Ok(data_stream
            .iter::<i8>()
            .map(std::convert::Into::into)
            .map(|s: f64| s / MAX_8_BIT)
            .collect::<Signal>()),
        16 => Ok(data_stream
            .iter::<i16>()
            .map(std::convert::Into::into)
            .map(|s: f64| s / MAX_16_BIT)
            .collect::<Signal>()),
        24 => Ok(data_stream
            .iter::<i32>()
            .map(|s| (s >> 8).into())
            .map(|s: f64| s / MAX_24_BIT)
            .collect::<Signal>()),
        32 => Ok(data_stream
            .iter::<i32>()
            .map(std::convert::Into::into)
            .map(|s: f64| s / MAX_32_BIT)
            .collect::<Signal>()),
        _ => Err(format!("Unknown bit depth: {depth} bit")),
    }
}
