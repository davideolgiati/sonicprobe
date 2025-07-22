use std::fs::File;

use flac::{ReadStream, Stream};

use crate::channel::Channel;
use crate::channel::channel_builder::ChannelBuilder;

const MAX_8_BIT: f32 = i8::MAX as f32;
const MAX_16_BIT: f32 = i16::MAX as f32;
const MAX_24_BIT: f32 = ((1 << 23) - 1) as f32;
const MAX_32_BIT: f32 = i32::MAX as f32;

pub struct FlacFile {
        // TODO: the channel numbers needs to be set looking at the file
        left: Channel,
        right: Channel,
        channels: u8,
        sample_rate: u32,
        bit_depth: u8,
}

impl FlacFile {
        pub fn new(mut data_stream: Stream<ReadStream<File>>) -> FlacFile {
                let bit_depth = data_stream.info().bits_per_sample;
                let channels = data_stream.info().channels;
                let sample_rate = data_stream.info().sample_rate;
                
                let mut left_channel_builder = ChannelBuilder::new(sample_rate);
                let mut right_channel_builder = ChannelBuilder::new(sample_rate);

                let mapped_stream = match bit_depth {
                        8 => data_stream.iter::<i8>().map(|s| s as f32 / MAX_8_BIT).collect::<Vec<f32>>(),
                        16 => data_stream.iter::<i16>().map(|s| s as f32 / MAX_16_BIT).collect::<Vec<f32>>(),
                        24 => data_stream.iter::<i32>().map(|s| s as f32 / MAX_24_BIT).collect::<Vec<f32>>(),
                        32 => data_stream.iter::<i32>().map(|s| s as f32 / MAX_32_BIT).collect::<Vec<f32>>(),
                        _ => panic!("Unknown bit depth"),
                };

                for pair in  mapped_stream.chunks_exact(2) {
                        left_channel_builder.add(pair[0]);
                        right_channel_builder.add(pair[1]);
                }

                FlacFile {
                        left: left_channel_builder.build(),
                        right: right_channel_builder.build(),
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

        pub fn channel_balance(&self) -> f32 {
                self.left.rms() - self.right.rms()
        }

        pub fn to_json_string(&self) -> String {
                let inner_tab: String = "\t".to_string();
                let output = [
                        format!("{}\"channel_count\": {},\n", inner_tab, self.channel_count()),
                        format!("{}\"sample_rate\": {},\n", inner_tab, self.sample_rate()),
                        format!("{}\"bit_depth\": {},\n", inner_tab, self.bit_depth()),
                        format!("{}\"channel_balance\": {},\n", inner_tab, self.channel_balance()),
                        format!("{}\"left\": {},\n", inner_tab, self.left.to_json_string(1)),
                        format!("{}\"right\": {}\n", inner_tab, self.right.to_json_string(1)),     
                ].concat();

                format!(
                        "{{\n{}}}",
                        output,
                )
        }
}