mod channel;
mod flac_file;
mod audio_utils;

extern crate flac;

use flac::StreamReader;
use std::fs::File;
use std::env;

use crate::channel::Channel;
use crate::flac_file::FlacFile;

fn print_file_details(file: &FlacFile) {
    let output = [
        format!("Channels: {}\n", file.channel_count()),
        format!("Sample Rate: {} Hz\n", file.sample_rate()),
        format!("Depth: {} bit\n", file.bit_depth()),
        format!("Channels balance: {:.2} db", file.channel_balance())
    ].concat();

    println!("{}", output);
}

fn print_channel_details(channel: &Channel, name: &str) {
    let output = [
        format!("{} channel:\n", name),
        format!("\tRMS: {:.2} db\n", channel.rms()),
        format!("\tPeak: {:.2} db\n", channel.peak()),
        format!("\tClipping: {:.3} %\n", channel.clip_samples_quota()),
        format!("\tDC Offset: {:.5}\n", channel.dc_offset()),
        format!("\tTrue Peak: {:.2} db\n", channel.true_peak()),
        format!("\tTrue Clipping: {:.3} %\n", channel.true_clip_samples_quota()),
        format!("\tCrest Factor: {:.2} db\n", channel.crest_factor()),
    ].concat();

    println!("{}", output);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No input file specified")
    }

    let input_file = &args[1];

    let flac_details = match StreamReader::<File>::from_file(input_file) {
        Ok(stream) => FlacFile::new(stream).await,
        Err(error)     => panic!("error: {:?}", error),
    };

    print_file_details(&flac_details);
    print_channel_details(flac_details.left(), "Left");
    print_channel_details(flac_details.right(), "Right");

    println!("{}", flac_details.to_json_string())
}
