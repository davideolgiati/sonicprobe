use std::fs::File;

use flac::{ReadStream, Stream};

use crate::channel::Channel;
use crate::channel::channel_builder::ChannelBuilder;

const MAX_INT: f64 = i16::MAX as f64;

pub struct FlacFile {
        // TODO: the channel numbers needs to be set looking at the file
        left: Channel,
        right: Channel,
        channels: u8,
        sample_rate: u32,
        bit_depth: u8,
}

impl FlacFile {
        pub async fn new(mut data_stream: Stream<ReadStream<File>>) -> FlacFile {
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

        pub fn bit_depth(&self) -> u8 {
                self.bit_depth
        }

        pub fn channel_balance(&self) -> f64 {
                self.left.rms() - self.right.rms()
        }
}