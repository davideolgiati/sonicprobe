mod flac_file;

extern crate flac;

use flac::StreamReader;
use std::fs::File;

use crate::flac_file::new_flac_file;

fn main() {
    let flac_details = match StreamReader::<File>::from_file("/home/davide/Musica/test1.flac") {
        Ok(stream) => new_flac_file(stream),
        Err(error)     => panic!("error: {:?}", error),
    };

    println!("Channels: {}\nSample Rate: {} Hz\nDepth: {} bit", flac_details.channels, flac_details.sample_rate, flac_details.bit_depth);
    println!("\nLEFT CHANNEL:");
    println!("DBFS: {:.2} db", flac_details.left.rms);
    println!("Peak: {:.2} db", flac_details.left.peak);
    println!("Samples clipping: {:.3} %", (flac_details.left.clip_sample_count as f64 / flac_details.left.samples_count as f64) * 100.0);
    println!("DC Offset: {:.5}", flac_details.left.dc_offset);
    println!("\nRIGHT CHANNEL:");
    println!("DBFS: {:.2} db", flac_details.right.rms);
    println!("Peak: {:.2} db", flac_details.right.peak);
    println!("Samples clipping: {:.3} %", (flac_details.right.clip_sample_count as f64 / flac_details.right.samples_count as f64) * 100.0);
    println!("DC Offset: {:.5}", flac_details.right.dc_offset);
}
