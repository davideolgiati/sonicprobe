mod channel;
mod flac_file;
mod audio_utils;

extern crate flac;

use flac::StreamReader;
use std::fs::File;

use crate::flac_file::new_flac_file;

#[tokio::main]
async fn main() {
    let flac_details = match StreamReader::<File>::from_file("/home/davide/Musica/test1.flac") {
        Ok(stream) => new_flac_file(stream).await,
        Err(error)     => panic!("error: {:?}", error),
    };

    println!("Channels: {}\nSample Rate: {} Hz\nDepth: {} bit", flac_details.channels, flac_details.sample_rate, flac_details.bit_depth);
    println!("\nLeft channel:");
    println!("\tDBFS: {:.2} db", flac_details.left.rms());
    println!("\tPeak: {:.2} db", flac_details.left.peak());
    println!("\tSamples clipping: {:.3} %", flac_details.left.clip_samples_quota());
    println!("\tDC Offset: {:.5}", flac_details.left.dc_offset());
    println!("\tCrest Factor: {:.2} db", flac_details.left.crest_factor());
    println!("\nRight channel:");
    println!("\tDBFS: {:.2} db", flac_details.right.rms());
    println!("\tPeak: {:.2} db", flac_details.right.peak());
    println!("\tSamples clipping: {:.3} %", flac_details.right.clip_samples_quota());
    println!("\tDC Offset: {:.5}", flac_details.right.dc_offset());
    println!("\tCrest Factor: {:.2} db", flac_details.right.crest_factor());
}
