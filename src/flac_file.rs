use std::fs::File;

use flac::{ReadStream, Stream};

use crate::channel::Channel;
use crate::channel::channel_builder::ChannelBuilder;
use crate::stereo_correlation_builder::StereoCorrelationBuilder;
use crate::true_bit_depth_builder::TrueBitDepthBuilder;

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
        true_bit_depth: u8,
        min_bit_depth: u8,
        max_bit_depth: u8,
        duration: f32,
        samples_count: u64,
        stereo_correlation: f32
}

impl FlacFile {
        pub fn new(mut data_stream: Stream<ReadStream<File>>) -> FlacFile {
                let bit_depth = data_stream.info().bits_per_sample;
                let channels = data_stream.info().channels;
                let sample_rate = data_stream.info().sample_rate;
                let samples_count = data_stream.info().total_samples;

                let mapped_stream = match bit_depth {
                        8 => data_stream.iter::<i8>().map(|s| s as f32 / MAX_8_BIT).collect::<Vec<f32>>(),
                        16 => data_stream.iter::<i16>().map(|s| s as f32 / MAX_16_BIT).collect::<Vec<f32>>(),
                        24 => data_stream.iter::<i32>().map(|s| s as f32 / MAX_24_BIT).collect::<Vec<f32>>(),
                        32 => data_stream.iter::<i32>().map(|s| s as f32 / MAX_32_BIT).collect::<Vec<f32>>(),
                        _ => panic!("Unknown bit depth"),
                };

                let (left_samples, right_samples): (Vec<f32>, Vec<f32>) = mapped_stream
                        .chunks_exact(2)
                        .map(|pair| (pair[0], pair[1]))
                        .unzip();
    
                let mut left_channel: Channel = Channel::empty();
                let mut right_channel: Channel = Channel::empty();
                let mut stereo_correlation: f32 = 0.0;
                let mut true_bit_depth: u8 = 0;
                let mut min_bit_depth: u8 = 0;
                let mut max_bit_depth: u8 = 0;

                rayon::scope(|s| {
                        s.spawn(|_| left_channel = ChannelBuilder::from_samples(&left_samples, sample_rate, samples_count));
                        s.spawn(|_| right_channel = ChannelBuilder::from_samples(&right_samples, sample_rate, samples_count));
                        s.spawn(|_| {
                                let mut stereo_correlation_builder = StereoCorrelationBuilder::new();
                                stereo_correlation_builder.add(&left_samples, &right_samples);
                                stereo_correlation = stereo_correlation_builder.build();
                        });
                        s.spawn(|_| {
                                let factor = match bit_depth {
                                        8 => MAX_8_BIT,
                                        16 => MAX_16_BIT,
                                        24 => MAX_24_BIT,
                                        32 => MAX_32_BIT,
                                        _ => panic!("Unknown bit depth"),
                                };
                                let mut true_bit_depth_builder = TrueBitDepthBuilder::new(bit_depth, samples_count);
                                true_bit_depth_builder.add(mapped_stream, factor);
                                (min_bit_depth, max_bit_depth, true_bit_depth) = true_bit_depth_builder.build();
                        });
                });

                FlacFile {
                        left: left_channel,
                        right: right_channel,
                        bit_depth,
                        channels,
                        sample_rate,
                        duration: samples_count as f32 / sample_rate as f32,
                        samples_count,
                        stereo_correlation,
                        true_bit_depth,
                        min_bit_depth,
                        max_bit_depth
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

        pub fn true_bit_depth(&self) -> u8 {
                self.true_bit_depth
        }

        pub fn min_bit_depth(&self) -> u8 {
                self.min_bit_depth
        }

        pub fn max_bit_depth(&self) -> u8 {
                self.max_bit_depth
        }

        pub fn rms_balance(&self) -> f32 {
                self.left.rms() - self.right.rms()
        }

        pub fn duration(&self) -> f32 {
                self.duration
        }

        pub fn samples_count(&self) -> u64 {
                self.samples_count
        }

        pub fn stereo_correlation(&self) -> f32 {
                self.stereo_correlation
        }

        pub fn to_json_string(&self) -> String {
                let inner_tab: String = "\t".to_string();
                let output = [
                        format!("{}\"channel_count\": {},\n", inner_tab, self.channel_count()),
                        format!("{}\"sample_rate\": {},\n", inner_tab, self.sample_rate()),
                        format!("{}\"bit_depth\": {},\n", inner_tab, self.bit_depth()),
                        format!("{}\"true_bit_depth\": {},\n", inner_tab, self.true_bit_depth()),
                        format!("{}\"min_bit_depth\": {},\n", inner_tab, self.min_bit_depth()),
                        format!("{}\"max_bit_depth\": {},\n", inner_tab, self.max_bit_depth()),
                        format!("{}\"duration\": {},\n",  inner_tab, self.duration()),
                        format!("{}\"samples_count\": {},\n",  inner_tab, self.samples_count()),   
                        format!("{}\"rms_balance\": {},\n", inner_tab, self.rms_balance()),
                        format!("{}\"stereo_correlation\": {},\n", inner_tab, self.stereo_correlation()),
                        format!("{}\"left\": {},\n", inner_tab, self.left.to_json_string(1)),
                        format!("{}\"right\": {}\n", inner_tab, self.right.to_json_string(1)),
                ].concat();

                format!(
                        "{{\n{}}}",
                        output,
                )
        }
}