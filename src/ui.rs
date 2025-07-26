mod filesystem;
mod audio;

use crate::{flac_file::FlacFile, ui::{audio::{format_db, format_hz, format_volt}, filesystem::{filename_from_path, get_formatted_file_size}}};

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

fn table_row(title: &str, left: &str, right: &str) -> String {
        format!("│  {:<23}   │    {:>12}   │     {:>12}   │", title, left, right)
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

    println!(
        "\n\n┌{}┬{}┬{}┐",
        "─".repeat(28),
        "─".repeat(19),
        "─".repeat(20)
    );
    println!("{}", table_row("CHANNEL ANALYSIS", "LEFT", "RIGHT"));
    println!("├{}┼{}┼{}┤", "─".repeat(28), "─".repeat(19), "─".repeat(20));
    println!("{}", table_row("RMS Level", &format_db(left.rms()), &format_db(right.rms())));
    println!("{}", table_row("Peak Level", &format_db(left.peak()), &format_db(right.peak())));
    println!("{}", table_row("True Peak", &format_db(left.true_peak()), &format_db(right.true_peak())));
    println!("{}", table_row("Crest Factor", &format_db(left.crest_factor()), &format_db(right.crest_factor())));
    println!("{}", table_row("DC Offset", &format_volt(left.dc_offset()), &format_volt(right.dc_offset())));
    println!("{}", table_row("Zero Crossing Rate", &format_hz(left.zero_crossing_rate().round() as u32), &format_hz(right.zero_crossing_rate().round() as u32)));
    println!("├{}┼{}┼{}┤", "─".repeat(28), "─".repeat(19), "─".repeat(20));
    println!(
        "│  {:<23}   │    {:>9.5}  %   │     {:>9.5}  %   │",
        "Clipping",
        left.clipping_samples_quota() * 100.0,
        right.clipping_samples_quota() * 100.0
    );
    println!(
        "│  {:<23}   │    {:>9.5}  %   │     {:>9.5}  %   │",
        "True Clipping",
        left.true_clipping_samples_quota() * 100.0,
        right.true_clipping_samples_quota() * 100.0
    );
    println!(
        "└{}┴{}┴{}┘\n\n",
        "─".repeat(28),
        "─".repeat(19),
        "─".repeat(20)
    );
}
