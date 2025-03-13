use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Mixtape",
    version = "0.1.0",
    about = "A Rust-based media conversion engine using FFmpeg",
    long_about = "Mixtape is a command-line media conversion tool that utilizes FFmpeg for format transformations."
)]
pub struct Args {
    #[arg(required = true, help = "Input and output file pairs")]
    pub files: Vec<String>,

    #[arg(short, long, help = "Force overwrite existing files")]
    pub force: bool,

    #[arg(short, long, help = "Simulate conversion without actually running FFmpeg")]
    pub dry_run: bool,

    #[arg(short, long, help = "Custom FFmpeg options")]
    pub options: Option<String>,
}
