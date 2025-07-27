mod audio;
mod entry;
mod filesystem;
mod table;

use crate::{
    flac_file::FlacFile,
    ui::{
        audio::{format_db, format_hz},
        entry::Entry,
        filesystem::{filename_from_path, get_formatted_file_size},
        table::Table,
    },
};

fn section_header(title: &str) -> String {
    let separator_len = 70 - title.len() - 4;
    format!(
        "── {} {}\n",
        title.to_ascii_uppercase(),
        "─".repeat(separator_len)
    )
}

fn seconds_to_minute_mark(duration: f32) -> String {
    let seconds = duration % 60_f32;
    let minutes = (duration - seconds) / 60_f32;

    format!("{:02.0}:{:02.0}", minutes, seconds)
}

pub fn format_percent(value: f32) -> String {
    if value > 0.0 {
        format!("+{:.5}  %", value * 100.0)
    } else if value == 0.0 {
        "0.00000  %".to_string()
    } else {
        format!("-{:.5}  %", value * 100.0)
    }
}

pub fn print_file_details(filename: &str, file: &FlacFile) {
    let left = file.left();
    let right = file.right();

    println!("{}", "=".repeat(70));
    println!("{:^70}", "SONICPROBE - AUDIO ANALYSIS REPORT");
    println!("{}\n", "=".repeat(70));

    println!("{}", section_header("FILE DETAILS"));
    println!("   {:<18} : {}", "Filename", filename_from_path(filename));
    println!("   {:<18} : {}", "Size", get_formatted_file_size(filename));
    println!("   {:<18} : {}", "Sample Count", file.samples_count());
    println!(
        "   {:<18} : {}",
        "Duration",
        seconds_to_minute_mark(file.duration())
    );
    println!(
        "   {:<18} : {} bit / {}",
        "Format",
        file.bit_depth(),
        format_hz(file.sample_rate())
    );
    println!(
        "   {:<18} : {} bit (Range {}-{})",
        "Bit depth usage",
        file.true_bit_depth(),
        file.min_bit_depth(),
        file.max_bit_depth()
    );

    println!("\n\n{}", section_header("STEREO FIELD ANALYSIS"));
    println!("   {:<18} :  {}", "Channels", file.channel_count());
    println!(
        "   {:<18} : {}",
        "RMS Balance (L/R)",
        format_db(file.rms_balance())
    );
    println!(
        "   {:<18} :  {:.2}",
        "Stereo Correlation",
        file.stereo_correlation()
    );

    let channels_details_table = Table::new()
        .add(
            Entry::new("CHANNEL ANALYSIS"),
            Entry::new("LEFT"),
            Entry::new("RIGHT"),
        )
        .add_section()
        .add(
            Entry::new("RMS Level"),
            Entry::from_db(left.rms()),
            Entry::from_db(right.rms()),
        )
        .add(
            Entry::new("Peak Level"),
            Entry::from_db(left.peak()),
            Entry::from_db(right.peak()),
        )
        .add(
            Entry::new("True Peak"),
            Entry::from_db(left.true_peak()),
            Entry::from_db(right.true_peak()),
        )
        .add(
            Entry::new("Crest Factor"),
            Entry::from_db(left.crest_factor()),
            Entry::from_db(right.crest_factor()),
        )
        .add(
            Entry::new("DC Offset"),
            Entry::from_volt(left.dc_offset()),
            Entry::from_volt(right.dc_offset()),
        )
        .add(
            Entry::new("Zero Crossing Rate"),
            Entry::from_hz(left.zero_crossing_rate()),
            Entry::from_hz(right.zero_crossing_rate()),
        )
        .add_section()
        .add(
            Entry::new("Clipping"),
            Entry::from_percent(left.clipping_samples_quota()),
            Entry::from_percent(right.clipping_samples_quota()),
        )
        .add(
            Entry::new("True Clipping"),
            Entry::from_percent(left.true_clipping_samples_quota()),
            Entry::from_percent(right.true_clipping_samples_quota()),
        )
        .build();

    println!("\n\n{}", channels_details_table)
}
