use std::fs::File;
use std::sync::Arc;
use std::thread;

use flac::{ReadStream, Stream};

use crate::builders::ClippingSamplesBuilder;
use crate::builders::PeakBuilder;
use crate::builders::StereoCorrelationBuilder;
use crate::builders::TrueBitDepthBuilder;
use crate::channel::Channel;
use crate::constants::MAX_16_BIT;
use crate::constants::MAX_24_BIT;
use crate::constants::MAX_32_BIT;
use crate::constants::MAX_8_BIT;
use crate::dsp::upsample;

type Signal = Arc<[f32]>;
type Frequency = u32;
type BitPrecision = u8;
type Milliseconds = f32;

pub struct FlacFile {
    left: Channel, // OK
    right: Channel, // OK
    /* Group next 4 */
    samples_per_channel: u64,
    sample_rate: Frequency,
    duration: Milliseconds,
    stereo_correlation: f32,
    channels: u8, // OK
    /* Group next 4 */
    depth: BitPrecision,
    true_depth: BitPrecision,
}

fn analyze(samples: Signal) -> (f32, usize) {
    let mut peak = f32::MIN;
    let mut clip_count = 0;

    rayon::scope(|s| {
        s.spawn(|_| peak = PeakBuilder::process(&samples));
        s.spawn(|_| clip_count = ClippingSamplesBuilder::process(&samples));
    });

    (peak, clip_count)
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
) -> std::thread::JoinHandle<(f32, usize)> {
    thread::spawn(move || analyze(upsample(data, original_sample_rate)))
}

impl FlacFile {
    pub fn new(data_stream: Stream<ReadStream<File>>) -> FlacFile {
        let channel_count = data_stream.info().channels;
        let sample_rate = data_stream.info().sample_rate;
        let depth = data_stream.info().bits_per_sample;
        let samples_per_channel = data_stream.info().total_samples;

        let signal = read_flac_file(data_stream, depth);

        let (left_samples, right_samples) = split_sample_array_into_channels(&signal);

        let left_upsample_worker = new_upsample_thread(left_samples.clone(), sample_rate);
        let right_upsample_worker = new_upsample_thread(right_samples.clone(), sample_rate);
        let left_worker = new_channel_thread(left_samples.clone(), sample_rate, samples_per_channel);
        let right_worker = new_channel_thread(right_samples.clone(), sample_rate, samples_per_channel);
        let stereo_correlation_worker = thread::spawn(move || StereoCorrelationBuilder::process(&left_samples, &right_samples));
        let bit_depth_worker = thread::spawn(move || TrueBitDepthBuilder::process(signal, depth));

        let true_bit_depth = bit_depth_worker.join().unwrap();
        let stereo_correlation = stereo_correlation_worker.join().unwrap();
        let mut left = left_worker.join().unwrap();
        let mut right = right_worker.join().unwrap();
        (left.true_peak, left.true_clipping_samples_count) = left_upsample_worker.join().unwrap();
        (right.true_peak, right.true_clipping_samples_count) = right_upsample_worker.join().unwrap();

        FlacFile {
            left,
            right,
            depth,
            channels: channel_count,
            sample_rate,
            duration: samples_per_channel as f32 / sample_rate as f32,
            samples_per_channel,
            stereo_correlation,
            true_depth: true_bit_depth
        }
    }

    pub fn left(&self) -> &Channel {
        &self.left
    }

    pub fn right(&self) -> &Channel {
        &self.right
    }

    pub fn channel_count(&self) -> u8 {
        self.channels
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn depth(&self) -> BitPrecision {
        self.depth
    }

    pub fn true_bit_depth(&self) -> u8 {
        self.true_depth
    }

    pub fn rms_balance(&self) -> f32 {
        self.left.rms() - self.right.rms()
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn samples_count(&self) -> u64 {
        self.samples_per_channel
    }

    pub fn stereo_correlation(&self) -> f32 {
        self.stereo_correlation
    }

    pub fn to_json_string(&self) -> String {
        let inner_tab: String = "\t".to_string();
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

        format!("{{\n{}}}", output,)
    }
}

fn split_sample_array_into_channels(samples: &Signal) -> (Signal, Signal) {
    let (left_vec, right_vec): (Vec<f32>, Vec<f32>) = samples
        .chunks_exact(2)
        .map(|pair| (pair[0], pair[1]))
        .unzip();

    (Arc::from(left_vec), Arc::from(right_vec))
}

fn read_flac_file(mut data_stream: Stream<ReadStream<File>>, depth: BitPrecision) -> Signal {
    match depth {
        8 => data_stream
            .iter::<i8>()
            .map(|s| s as f32 / MAX_8_BIT)
            .collect::<Signal>(),
        16 => data_stream
            .iter::<i16>()
            .map(|s| s as f32 / MAX_16_BIT)
            .collect::<Signal>(),
        24 => data_stream
            .iter::<i32>()
            .map(|s| s as f32 / MAX_24_BIT)
            .collect::<Signal>(),
        32 => data_stream
            .iter::<i32>()
            .map(|s| s as f32 / MAX_32_BIT)
            .collect::<Signal>(),
        _ => panic!("Unknown bit depth"),
    }
}
