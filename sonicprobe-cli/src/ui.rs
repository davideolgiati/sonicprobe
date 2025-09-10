mod audio;
mod entry;
mod filesystem;
mod table;
mod section;

use sonicprobe_core::audio_file::AudioFile;

use crate::{
    ui::{
        entry::Entry, filesystem::{filename_from_path, get_formatted_file_size}, section::Section, table::Table
    },
};



fn seconds_to_minute_mark(duration: usize) -> String {
    let seconds = duration % 60;
    let minutes = (duration - seconds) / 60;

    format!("{minutes:02.0}:{seconds:02.0}")
}

pub fn print_file_details(filepath: &str, file: &AudioFile) {
    let left = file.left;
    let right = file.right;
    let filename = filename_from_path(filepath).map_or_else(|| filepath.to_owned(), |value| value);
    let formatted_size = get_formatted_file_size(filepath).unwrap_or_else(|_| "?".to_owned());

    println!("{}", "=".repeat(70));
    println!("{:^70}", "SONICPROBE - AUDIO ANALYSIS REPORT");
    println!("{}\n", "=".repeat(70));

    let file_details = Section::new("FILE DETAILS")
        .add("Filename", Entry::from(filename))
        .add("Size", Entry::from(formatted_size))
        .add("Sample Count", Entry::from(file.samples_per_channel * usize::from(file.channels)))
        .add("Duration", Entry::from(seconds_to_minute_mark(file.duration)))
        .add("Sample Rate", Entry::from(file.sample_rate.description().to_owned()))
        .add("Bit Depth", Entry::from(file.depth.description().to_owned()))
        .add("Bit depth usage", Entry::from_bit(file.true_depth))
        .build();

    let stereo_field_analisys = Section::new("STEREO FIELD ANALYSIS")
        .add("Channels", Entry::from(file.channels as usize))
        .add("RMS Balance (L/R)",Entry::from(file.rms_balance()))
        .add("Stereo Correlation", Entry::from_percent(file.stereo_correlation * 100.0))
        .build();

    let channels_details_table = Table::new(left, right)
        .set_headers("CHANNEL ANALYSIS", "LEFT", "RIGHT")
        .add_section()
        .add("RMS Level", |c| Entry::from(c.rms()))
        .add("Peak Level", |c| Entry::from(c.peak()))
        .add("True Peak", |c| Entry::from(c.true_peak()))
        .add("Crest Factor", |c| Entry::from(c.crest_factor()))
        .add("DC Offset", |c| Entry::from_volt(c.dc_offset()))
        .add("Zero Crossing Rate", |c| {
            Entry::from_hz(c.zero_crossing_rate())
        })
        .add("Dynamic Range", |c| Entry::from(c.dr()))
        .add_section()
        .add("Clipping", |c| {
            Entry::from_percent(c.clipping_samples_ratio() * 100.0)
        })
        .add("True Clipping", |c| {
            Entry::from_percent(c.true_clipping_samples_ratio() * 100.0)
        })
        .build();

    println!("{file_details}");
    println!("{stereo_field_analisys}");
    println!("{channels_details_table}");
}
