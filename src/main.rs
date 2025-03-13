use clap::Parser;
use std::sync::Arc;

mod args;
mod ffmpeg;
mod logger;
mod utils;

use crate::args::Args;
use crate::ffmpeg::{ check_ffmpeg, process_conversions };
use crate::logger::log_error;

fn main() {
    let args = Arc::new(Args::parse());

    if !check_ffmpeg() {
        eprintln!("Error: FFmpeg is not installed. Please install it and try again.");
        log_error("FFmpeg is not installed.");
        std::process::exit(1);
    }

    if args.files.len() % 2 != 0 {
        eprintln!("Error: Each input file must have a corresponding output file.");
        log_error("Mismatched input/output file count.");
        std::process::exit(1);
    }

    process_conversions(args);
}
