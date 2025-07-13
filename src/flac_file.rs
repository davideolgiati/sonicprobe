use std::fs::File;

use flac::{ReadStream, Stream};

const MAX_INT: f64 = i16::MAX as f64;
const CLIP_THRESH: f64 = 0.999_999;

pub struct Channel {
        pub rms: f64,
        pub peak: f64,
        pub clip_sample_count: i32,
        pub dc_offset: f64,
        pub samples_count: i32,
}

pub struct FlacFile {
        // TODO: the channel numbers needs to be set looking at the file
        pub left: Channel,
        pub right: Channel,
        pub channels: u8,
        pub sample_rate: u32,
        pub bit_depth: u8,
}

fn compute_rms(samples: &[f64]) -> f64 {
        let processed_samples = {
                let mut new_vec: Vec<f64> = samples.iter()
                        .map(|sample| sample.powi(2))
                        .collect();
                new_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

                new_vec
        };

        let sample_sum = processed_samples.iter().sum::<f64>();
        let sample_avarage = sample_sum / samples.len() as f64;

        to_dbfs(sample_avarage.sqrt())
}

fn new_channel_from_samples(samples: Vec<f64>) -> Channel {
        let sorted_samples: Vec<f64> = {
                let mut new_vec = samples.clone();
                new_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

                new_vec
        };

        let rms: f64 = compute_rms(&samples);
        let peak: f64 = to_dbfs(sorted_samples[sorted_samples.len()-1]);

        let clip_sample_count: i32 = samples.iter()
                .filter(|&&x| x >= CLIP_THRESH || x <= -CLIP_THRESH)
                .count() as i32;

        let dc_offset: f64 = sorted_samples.iter()
                .sum::<f64>() / samples.len() as f64;

        Channel {
                rms, 
                peak, 
                clip_sample_count, 
                dc_offset,
                samples_count: samples.len() as i32
        }
}

pub fn new_flac_file(mut data_stream: Stream<ReadStream<File>>) -> FlacFile {
        let samples: Vec<f64> = data_stream.iter::<i16>() // TODO: this needs to be set looking at the file
                .map(|sample| sample as f64 / MAX_INT)
                .collect();

        let samples_per_channel = samples.len() / 2;

        let mut left  = Vec::with_capacity(samples_per_channel);
        let mut right = Vec::with_capacity(samples_per_channel);
        
        for chunk in samples.chunks_exact(2) {
                left.push(chunk[0]);
                right.push(chunk[1]);
        }
        
        FlacFile {
                left: new_channel_from_samples(left),
                right: new_channel_from_samples(right),
                bit_depth: data_stream.info().bits_per_sample,
                channels: data_stream.info().channels,
                sample_rate: data_stream.info().sample_rate
        }
}

fn to_dbfs(rms: f64) -> f64 {
        20.0 * rms.log10()
}