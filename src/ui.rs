mod audio;
mod entry;
mod filesystem;
mod table;

use crate::{
    audio_file::AudioFile,
    ui::{
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

pub fn print_file_details(filename: &str, file: &AudioFile) {
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
        "   {:<18} : {} / {}",
        "Format",
        Entry::from_bit(file.depth()).formatted(),
        Entry::from_hz(file.sample_rate() as f32).formatted()
    );
    println!(
        "   {:<18} : {} bit",
        "Bit depth usage",
        file.true_bit_depth()
    );

    println!("\n\n{}", section_header("STEREO FIELD ANALYSIS"));
    println!("   {:<18} :  {}", "Channels", file.channel_count());
    println!(
        "   {:<18} : {}",
        "RMS Balance (L/R)",
        Entry::from_db(file.rms_balance()).formatted()
    );
    println!(
        "   {:<18} :  {:.2}",
        "Stereo Correlation",
        file.stereo_correlation()
    );

    let channels_details_table = Table::new(*left, *right)
        .set_headers("CHANNEL ANALYSIS", "LEFT", "RIGHT")
        .add_section()
        .add("RMS Level", |c| Entry::from_db(c.rms()))
        .add("Peak Level", |c| Entry::from_db(c.peak()))
        .add("True Peak", |c| Entry::from_db(c.true_peak()))
        .add("Crest Factor", |c| Entry::from_db(c.crest_factor()))
        .add("DC Offset", |c| Entry::from_volt(c.dc_offset()))
        .add("Zero Crossing Rate", |c| {
            Entry::from_hz(c.zero_crossing_rate())
        })
        .add("Dynamic Range", |c| Entry::from_dr(c.dr()))
        .add_section()
        .add("Clipping", |c| {
            Entry::from_percent(c.clipping_samples_quota())
        })
        .add("True Clipping", |c| {
            Entry::from_percent(c.true_clipping_samples_quota())
        })
        .build();

    println!("\n\n{}", channels_details_table)
}
