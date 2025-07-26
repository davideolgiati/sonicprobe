mod channel;
mod cli_args;
mod flac_file;
mod audio_utils;
mod output_format;
mod circular_buffer;
mod stereo_correlation_builder;
mod true_bit_depth_builder;

extern crate flac;

use std::{env, fs};
use std::fs::File;
use std::path::Path;
use flac::StreamReader;

use crate::cli_args::CliArgs;
use crate::flac_file::FlacFile;
use crate::output_format::OutputFormat;

fn format_db(value: f32) -> String {
    if value > 0.0 {
        format!("+{:.2} dB", value)
    } else if value == 0.0 {
        "0.00 dB".to_string()
    } else {
        format!("{:.2} dB", value)
    }
}

fn filename_from_path(filepath: &str) -> String {
    Path::new(filepath)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
        .unwrap()
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn get_formatted_file_size<P: AsRef<Path>>(path: P) -> String {
    let metadata = fs::metadata(path).unwrap();
    let size = metadata.len();
    format_file_size(size)
}

fn format_volt(value: f32) -> String {
    if value > 0.0 {
        format!("+{:.5}  V", value)
    } else if value == 0.0 {
        "0.00000  V".to_string()
    } else {
        format!("{:.5}  V", value)
    }
}

fn format_hz(value: u32) -> String {
    format!("{} Hz", value)
}

fn print_file_details(filename: &str, file: &FlacFile) {
    let seconds = file.duration() % 60_f32;
    let minutes = (file.duration() - seconds) / 60_f32;
    let left = file.left();
    let right = file.right();

    println!("{}", "=".repeat(70));
    println!("{:^70}", "SONICPROBE - AUDIO ANALYSIS REPORT");
    println!("{}\n", "=".repeat(70));
    
    println!("── FILE DETAILS {}\n", "─".repeat(54));
    println!("   {:<18} : {}", "Filename", filename_from_path(filename));
    println!("   {:<18} : {}", "Size", get_formatted_file_size(filename));
    println!("   {:<18} : {}", "Sample Count", file.samples_count());
    println!("   {:<18} : {:02.0}:{:02.0}", "Duration", minutes, seconds);
    println!("   {:<18} : {} bit / {}", "Format", file.bit_depth(), format_hz(file.sample_rate()));
    println!("   {:<18} : {} bit (Range {}-{})", "Bit depth usage", file.true_bit_depth(), file.min_bit_depth(),file.max_bit_depth());
    
    println!("\n\n── STEREO FIELD ANALYSIS {}\n", "─".repeat(45));
    println!("   {:<18} :  {}", "Channels", file.channel_count());
    println!("   {:<18} : {}", "RMS Balance (L/R)", format_db(file.rms_balance()));
    println!("   {:<18} :  {:.2}", "Stereo Correlation", file.stereo_correlation());
    
    println!("\n\n┌{}┬{}┬{}┐", "─".repeat(28), "─".repeat(19), "─".repeat(20));
    println!("│  {:<23}   │    {:>12}   │     {:>12}   │", "CHANNEL ANALYSIS", "LEFT", "RIGHT");
    println!("├{}┼{}┼{}┤", "─".repeat(28), "─".repeat(19), "─".repeat(20));
    println!("│  {:<23}   │    {:>12}   │     {:>12}   │", "RMS Level", format_db(left.rms()), format_db(right.rms()));
    println!("│  {:<23}   │    {:>12}   │     {:>12}   │", "Peak Level", format_db(left.peak()), format_db(right.peak()));
    println!("│  {:<23}   │    {:>12}   │     {:>12}   │", "True Peak", format_db(left.true_peak()), format_db(right.true_peak()));
    println!("│  {:<23}   │    {:>12}   │     {:>12}   │", "Crest Factor", format_db(left.crest_factor()), format_db(right.crest_factor()));
    println!("│  {:<23}   │    {:>12}   │     {:>12}   │", "DC Offset", format_volt(left.dc_offset()), format_volt(right.dc_offset()));
    println!("│  {:<23}   │    {:>12}   │     {:>12}   │", "Zero Crossing Rate", format_hz(left.zero_crossing_rate().round() as u32), format_hz(right.zero_crossing_rate().round() as u32));
    println!("├{}┼{}┼{}┤", "─".repeat(28), "─".repeat(19), "─".repeat(20));
    println!("│  {:<23}   │    {:>9.5}  %   │     {:>9.5}  %   │", "Clipping", left.clipping_samples_quota() * 100.0, right.clipping_samples_quota() * 100.0);
    println!("│  {:<23}   │    {:>9.5}  %   │     {:>9.5}  %   │", "True Clipping", left.true_clipping_samples_quota() * 100.0, right.true_clipping_samples_quota() * 100.0);
    println!("└{}┴{}┴{}┘\n\n", "─".repeat(28), "─".repeat(19), "─".repeat(20));

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
