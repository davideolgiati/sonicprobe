use std::fs::File;

use flac::{ReadStream, Stream};

use crate::channel_builder::ChannelBuilder;

const MAX_INT: f64 = i16::MAX as f64;


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

pub async fn new_flac_file(mut data_stream: Stream<ReadStream<File>>) -> FlacFile {
        let mut left_channel_builder = ChannelBuilder::new();
        let mut right_channel_builder = ChannelBuilder::new();

        for (counter, sample) in data_stream.iter::<i16>().enumerate() {
                match counter % 2 {
                        0 => left_channel_builder.add(sample as f64 / MAX_INT).await,
                        _ => right_channel_builder.add(sample as f64 / MAX_INT).await,
                }
        }

        let bit_depth = data_stream.info().bits_per_sample;
        let channels = data_stream.info().channels;
        let sample_rate = data_stream.info().sample_rate;

        FlacFile {
                left: left_channel_builder.build().await,
                right: right_channel_builder.build().await,
                bit_depth,
                channels,
                sample_rate
        }
}