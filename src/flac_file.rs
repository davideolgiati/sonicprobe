use std::fs::File;

use flac::{ReadStream, Stream};

const MAX_INT: f64 = i16::MAX as f64;
const CLIP_THRESH: f64 = 0.999_999;

pub struct Channel {
        pub rms: f64,
        pub peak: f64,
        pub clip_sample_count: i32,
        pub dc_offset: f64,
}

pub struct FlacFile {
        // TODO: the channel numbers needs to be set looking at the file
        pub left: Channel,
        pub right: Channel,
        pub channels: u8,
        pub sample_rate: u32,
        pub bit_depth: u8,
}



fn new_channel_from_samples(samples: Vec<f64>) -> Channel {
        let rms: f64 = to_dbfs((samples.iter()
                .map(|sample| sample.powi(2))
                .sum::<f64>() / samples.len() as f64)
                .sqrt());

        let peak: f64 = samples.iter()
                .max_by(|a, b| a.total_cmp(b))
                .map(|value| to_dbfs(*value))
                .unwrap();

        let clip_sample_count: i32 = samples.iter()
                .filter(|&&x| x >= CLIP_THRESH || x <= -CLIP_THRESH)
                .count() as i32;

        let dc_offset: f64 = samples.iter()
                .sum::<f64>() / samples.len() as f64;

        Channel {
                rms, 
                peak, 
                clip_sample_count, 
                dc_offset 
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