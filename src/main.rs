mod channel;
mod cli_args;
mod flac_file;
mod audio_utils;
mod output_format;
mod circular_buffer;
mod stereo_correlation_builder;

extern crate flac;

use std::env;
use std::fs::File;
use flac::StreamReader;

use crate::cli_args::CliArgs;
use crate::flac_file::FlacFile;
use crate::output_format::OutputFormat;

fn print_file_details(filename: &String, file: &FlacFile) {
    let seconds = file.duration() % 60_f32;
    let minutes = (file.duration() - seconds) / 60_f32;
    let left = file.left();
    let right = file.right();

    println!("{}", "=".repeat(70));
    println!("SONICPROBE AUDIO ANALYSIS REPORT");
    println!("{}\n", "=".repeat(70));
    
    println!("── FILE DETAILS {}\n", "─".repeat(54));
    println!("   {:<18} : {}", "Filename", *filename);
    println!("   {:<18} : {:02.0}:{:02.0}", "Duration", minutes, seconds);
    println!("   {:<18} : {} bit / {:.1} kHz", "Format", file.bit_depth(), file.sample_rate() as f32 / 1000.0);
    println!("   {:<18} : {}", "Channels", file.channel_count());
    println!("   {:<18} : {}\n", "Sample Count", file.samples_count());
    
    println!("\n── STEREO FIELD ANALYSIS {}\n", "─".repeat(45));
    println!("   {:<18} : {:.2} dB", "RMS Balance (L/R)", file.rms_balance());
    println!("   {:<18} : {:.2}\n", "Stereo Correlation", file.stereo_correlation());
    
    println!("\n{}", "─".repeat(70));
    println!("       {:<23} |      {:>8}  |     {:>8}", "CHANNEL ANALYSIS", "LEFT", "RIGHT");
    println!("{}", "─".repeat(70));
    
    println!("       {:<23} |   {:>8.2} dB  |  {:>8.2} dB", "RMS Level", left.rms(), right.rms());
    println!("       {:<23} |   {:>8.2} dB  |  {:>8.2} dB", "Peak Level", left.peak(), right.peak());
    println!("       {:<23} |   {:>8.2} dB  |  {:>8.2} dB", "True Peak", left.true_peak(), right.true_peak());
    println!("       {:<23} |   {:>8.2} dB  |  {:>8.2} dB", "Crest Factor", left.crest_factor(), right.crest_factor());
    println!("       {:<23} |      {:>8.5}  |     {:>8.5}", "DC Offset", left.dc_offset(), right.dc_offset());
    println!("       {:<23} |   {:>8.0} Hz  |  {:>8.0} Hz", "Zero Crossing Rate", left.zero_crossing_rate(), right.zero_crossing_rate());
    println!("{}", "─".repeat(70));
    println!("       {:<23} |     {:>8.2}%  |    {:>8.2}%", "Sample Clipping", left.clipping_samples_quota() * 100.0, right.clipping_samples_quota() * 100.0);
    println!("       {:<23} |     {:>8.2}%  |    {:>8.2}%", "True Peak Clipping", left.true_clipping_samples_quota() * 100.0, right.true_clipping_samples_quota() * 100.0);
    println!("{}\n", "─".repeat(70));
    println!("{}", "=".repeat(70));

}

fn main() {
    let cli_input: Vec<String> = env::args().collect();
    let args: CliArgs = CliArgs::new(&cli_input);

    let flac_details = match StreamReader::<File>::from_file(args.file_path()) {
        Ok(stream) => FlacFile::new(stream),
        Err(error)     => panic!("error: {:?}", error),
    };

    if *args.output_format() == OutputFormat::Json {
        println!("{}", flac_details.to_json_string())
    } else {
        print_file_details(args.file_path(), &flac_details);
    }
}
