use std::fs::File;
use std::sync::Arc;
use std::thread;

use claxon::FlacReader;

use crate::dsp::analysis::bit_depth::calculate_true_depth;
use crate::dsp::analysis::stereo_correlation::calculate_stereo_correlation;
use crate::model::audio_file::AudioFile;
use crate::model::builders::channel_builder::ChannelBuilder;
use crate::model::builders::stereo_signal_builder::stereo_signal_from_flac;
use crate::model::sonicprobe_error::SonicProbeError;

pub fn audio_file_form_stream(stream: FlacReader<File>) -> Result<AudioFile, SonicProbeError> {
    let stereo_signal = stereo_signal_from_flac(stream)?;

    let left_thread_handle = thread::spawn({
        let left_channel = Arc::clone(&stereo_signal.left);
        let sample_rate = stereo_signal.sample_rate;
        move || ChannelBuilder::new(&left_channel, sample_rate).build()
    });

    let right_thread_handle = thread::spawn({
        let right_channel = Arc::clone(&stereo_signal.right);
        let sample_rate = stereo_signal.sample_rate;
        move || ChannelBuilder::new(&right_channel, sample_rate).build()
    });

    let true_bit_depth = calculate_true_depth(&stereo_signal)?;

    let samples_per_channel = stereo_signal.samples_per_channel();

    let left = left_thread_handle.join()??;
    let right = right_thread_handle.join()??;

    let stereo_correlation = calculate_stereo_correlation(&stereo_signal.left, &stereo_signal.right);

    Ok(AudioFile {
        left,
        right,
        channels: 2,
        stereo_correlation,
        true_depth: true_bit_depth,
        depth: stereo_signal.depth,
        sample_rate: stereo_signal.sample_rate,
        samples_per_channel,
        duration: samples_per_channel / stereo_signal.sample_rate.to_hz(),
    })
}
