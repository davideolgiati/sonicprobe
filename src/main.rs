extern crate flac;

use flac::StreamReader;
use std::fs::File;

const MAX_INT: f64 = i16::MAX as f64;

fn to_dbfs(rms: f64) -> f64 {
    20.0 * rms.log10()
}

fn main() {
    match StreamReader::<File>::from_file("/home/davide/Musica/test.flac") {
        Ok(mut stream) => {
            let info = stream.info();
            let samples: Vec<f64> = stream.iter::<i16>()
                .map(|sample| sample as f64 / MAX_INT)
                .collect();

            let rms: f64 = (samples.iter()
                .map(|sample| sample.powi(2))
                .sum::<f64>() / info.total_samples as f64)
                .sqrt();

            let peak: f64 = samples.iter().max_by(|a, b| a.total_cmp(b)).map(|value| to_dbfs(*value)).unwrap();

            let clip_count: f64 = (samples.iter()
                .map(|sample| {
                    if to_dbfs(*sample) > 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                })
                .sum::<f64>() / info.total_samples as f64) * 100.0;
        
            let dbfs_rms = to_dbfs(rms);
        
            let dc_offset: f64 = samples.iter().sum::<f64>() / samples.len() as f64;

            println!("Channels: {}\nSample Rate: {} Hz\nDepth: {} bit", info.channels, info.sample_rate, info.bits_per_sample);
            println!("DBFS: {:.2} db", dbfs_rms);
            println!("Peak: {:.2} db", peak);
            println!("Samples clipping: {:.2} %", clip_count);
            println!("DC Offset: {:.5}", dc_offset);
        }
        Err(error)     => println!("error: {:?}", error),
    }
}
