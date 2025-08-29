use std::fs::File;
use std::sync::Arc;
use std::thread;

use claxon::FlacReader;

use crate::dsp::analysis::bit_depth::calculate_actual_depth;
use crate::dsp::analysis::stereo_correlation::calculate_stereo_correlation;
use crate::model::audio_file::AudioFile;
use crate::model::builders::channel_builder::ChannelBuilder;
use crate::model::builders::stereo_signal_builder::stereo_signal_from_flac;
use crate::model::sonicprobe_error::SonicProbeError;

pub fn audio_file_form_stream(stream: FlacReader<File>) -> Result<AudioFile, SonicProbeError> {
    let source = stereo_signal_from_flac(stream)?;

    let left_handle = thread::spawn({
        let left_data = Arc::clone(&source.left);
        let sample_rate = source.sample_rate;
        move || ChannelBuilder::new(&left_data, sample_rate).build()
    });

    let right_handle = thread::spawn({
        let right_data = Arc::clone(&source.right);
        let sample_rate = source.sample_rate;
        move || ChannelBuilder::new(&right_data, sample_rate).build()
    });

    let true_bit_depth = calculate_actual_depth(&source)?;

    let signed_sample_count: i64 = source.samples_per_channel.try_into()?;

    let left = left_handle.join()??;
    let right = right_handle.join()??;

    let stereo_correlation = calculate_stereo_correlation(&source.left, &source.right);

    Ok(AudioFile {
        left,
        right,
        channels: 2,
        stereo_correlation,
        true_depth: true_bit_depth,
        depth: source.depth,
        sample_rate: source.sample_rate,
        samples_per_channel: source.samples_per_channel,
        duration: signed_sample_count / i64::from(source.sample_rate.to_hz()),
    })
}
