// Add these at the top of your main.rs or lib.rs

// === CLEAN CODE & SOLID PRINCIPLES ===
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
//
// Code organization and readability
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::similar_names)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::large_enum_variant)]
#![warn(clippy::struct_excessive_bools)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::too_many_arguments)]
//
// Documentation and naming
#![warn(clippy::inconsistent_struct_constructor)]
//
// Error handling (clean code principle)
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
//#![warn(clippy::panic)]
#![warn(clippy::unimplemented)]
#![warn(clippy::todo)]
#![warn(clippy::unreachable)]
//#![warn(clippy::missing_errors_doc)]
//#![warn(clippy::missing_panics_doc)]
//
// === CORRECTNESS & SAFETY ===
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
//
// Memory safety
#![warn(clippy::mem_forget)]
#![warn(clippy::mem_replace_with_uninit)]
#![warn(clippy::uninit_assumed_init)]
//#![warn(clippy::multiple_unsafe_ops_per_block)]
//#![warn(clippy::undocumented_unsafe_blocks)]
//
// Logic errors
#![warn(clippy::overly_complex_bool_expr)]
#![warn(clippy::suspicious_else_formatting)]
#![warn(clippy::suspicious_operation_groupings)]
#![warn(clippy::float_cmp)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::lossy_float_literal)]
//
// Iterator and collection correctness
#![warn(clippy::iter_skip_next)]
#![warn(clippy::iter_nth_zero)]
#![warn(clippy::iter_next_slice)]
#![warn(clippy::suspicious_map)]
#![warn(clippy::map_entry)]
//
// === PERFORMANCE ===
#![warn(clippy::perf)]
//
// String operations
#![warn(clippy::string_add)]
#![warn(clippy::string_add_assign)]
#![warn(clippy::string_slice)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
//
// Collection performance
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::single_char_pattern)]
#![warn(clippy::single_char_add_str)]
#![warn(clippy::unnecessary_to_owned)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::redundant_clone)]
//
// Allocation optimization
#![warn(clippy::vec_init_then_push)]
#![warn(clippy::extend_with_drain)]
#![warn(clippy::stable_sort_primitive)]
#![warn(clippy::unnecessary_sort_by)]
//
// Iteration performance
#![warn(clippy::needless_collect)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::filter_map_next)]
#![warn(clippy::flat_map_option)]
//
// === STYLE & IDIOMATIC RUST ===
#![warn(clippy::style)]
//
// Type annotations and casting
#![warn(clippy::unnecessary_cast)]
//#![warn(clippy::cast_lossless)]
//#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
//#![warn(clippy::cast_precision_loss)]
#![warn(clippy::cast_sign_loss)]
//
// Pattern matching
#![warn(clippy::match_same_arms)]
#![warn(clippy::redundant_pattern_matching)]
#![warn(clippy::wildcard_enum_match_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
//
// Function design
#![warn(clippy::must_use_candidate)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::unnecessary_wraps)]
#![warn(clippy::result_large_err)]
//
// === RESTRICT PROBLEMATIC PATTERNS ===
//#![warn(clippy::print_stderr)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
//
// === SPECIFIC SOLID PRINCIPLE ADHERENCE ===
// Single Responsibility
#![warn(clippy::type_complexity)]
#![warn(clippy::large_types_passed_by_value)]
//
// Open/Closed & Interface Segregation
#![warn(clippy::enum_variant_names)]
#![warn(clippy::module_inception)]
//
// Dependency Inversion
#![warn(clippy::new_without_default)]
#![warn(clippy::should_implement_trait)]
//
// === OPTIONAL: VERY STRICT (uncomment if desired) ===
//#![warn(clippy::arithmetic_side_effects)] // Prevents overflow in debug
//#![warn(clippy::float_arithmetic)] // Prevents float precision issues
//#![warn(clippy::as_conversions)] // Forces explicit error handling
#![warn(clippy::indexing_slicing)] // Prevents panic on out-of-bounds

mod audio_file;
mod audio_utils;
mod cli_args;
mod constants;
mod dsp;
mod output_format;
mod ui;

extern crate flac;

use flac::StreamReader;
use std::fs::File;
use std::{env, process};

use crate::audio_file::AudioFile;
use crate::cli_args::CliArgs;
use crate::output_format::OutputFormat;
use crate::ui::print_file_details;

fn main() {
    let cli_input: Vec<String> = env::args().collect();
    let args: CliArgs = CliArgs::new(&cli_input);

    let flac_details = match StreamReader::<File>::from_file(args.file_path()) {
        Ok(stream) => AudioFile::new(stream),
        Err(error) => {
            println!("error while opening {} : {:?}", args.file_path(), error);
            process::exit(1);
        }
    };

    if *args.output_format() == OutputFormat::Json {
        println!("{}", flac_details.to_json_string());
    } else {
        print_file_details(args.file_path(), &flac_details);
    }
}
