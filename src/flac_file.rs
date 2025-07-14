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

async fn compute_rms(samples: &[f64]) -> f64 {
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

async fn sort_samples(samples: &[f64]) -> Vec<f64> {
        let mut new_vec = samples.to_vec();
        new_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        new_vec.to_vec()
}

async fn new_channel_from_samples(samples: Vec<f64>) -> Channel {
        let sorted_samples_promise = sort_samples(&samples);

        let rms = compute_rms(&samples);
        let clip_sample_count = count_clip_samples(&samples);

        let sorted_samples = sorted_samples_promise.await;

        let dc_offset = compute_dc_offset(&sorted_samples);
        let peak: f64 = to_dbfs(sorted_samples[sorted_samples.len()-1]);

        Channel {
                rms: rms.await, 
                peak, 
                clip_sample_count: clip_sample_count.await, 
                dc_offset: dc_offset.await,
                samples_count: samples.len() as i32
        }
}

async fn compute_dc_offset(sorted_samples: &[f64]) -> f64 {
    let dc_offset: f64 = sorted_samples.iter()
            .sum::<f64>() / sorted_samples.len() as f64;
    dc_offset
}

async fn count_clip_samples(samples: &[f64]) -> i32 {
    let clip_sample_count: i32 = samples.iter()
            .filter(|&&x| x >= CLIP_THRESH || x <= -CLIP_THRESH)
            .count() as i32;
    clip_sample_count
}

fn convert_samples_stream(data_stream: &mut Stream<ReadStream<File>>) -> Vec<f64> {
        data_stream.iter::<i16>() // TODO: this needs to be set looking at the file
                .map(|sample| sample as f64 / MAX_INT)
                .collect::<Vec<f64>>()
}

pub async fn new_flac_file(mut data_stream: Stream<ReadStream<File>>) -> FlacFile {
        let samples: Vec<f64> = convert_samples_stream(&mut data_stream);
        let samples_per_channel = samples.len() / 2;

        let (left, right) = split_channels(samples, samples_per_channel);
        
        let left_channel_details = new_channel_from_samples(left);
        let right_channel_details = new_channel_from_samples(right);

        let bit_depth = data_stream.info().bits_per_sample;
        let channels = data_stream.info().channels;
        let sample_rate = data_stream.info().sample_rate;

        FlacFile {
                left: left_channel_details.await,
                right: right_channel_details.await,
                bit_depth,
                channels,
                sample_rate
        }
}

fn split_channels(samples: Vec<f64>, samples_per_channel: usize) -> (Vec<f64>, Vec<f64>) {
    let mut left  = Vec::with_capacity(samples_per_channel);
    let mut right = Vec::with_capacity(samples_per_channel);
        
    for chunk in samples.chunks_exact(2) {
            left.push(chunk[0]);
            right.push(chunk[1]);
    }

    (left, right)
}

fn to_dbfs(rms: f64) -> f64 {
        20.0 * rms.log10()
}