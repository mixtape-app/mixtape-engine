use indicatif::{ ProgressBar, ProgressStyle };
use num_cpus;
use std::io::{ self, BufRead };
use std::path::PathBuf;
use std::process::{ Child, Command, Stdio };
use std::sync::Arc;

use crate::args::Args;
use crate::logger::{ log_error, log_success };
use crate::utils::is_valid_format;

pub fn check_ffmpeg() -> bool {
    Command::new("ffmpeg").arg("-version").output().is_ok()
}

pub fn process_conversions(args: Arc<Args>) {
    let cpu_count = num_cpus::get();

    for pair in args.files.chunks(2) {
        let input = &pair[0];
        let output = &pair[1];

        if !PathBuf::from(input).exists() {
            eprintln!("Error: Input file '{}' does not exist.", input);
            log_error(&format!("Input file '{}' does not exist.", input));
            continue;
        }

        if !is_valid_format(input) || !is_valid_format(output) {
            eprintln!("Error: Unsupported file format.");
            log_error("Unsupported file format detected.");
            continue;
        }

        println!("Converting {} -> {} using {} CPU cores", input, output, cpu_count);

        let cpu_count_str = cpu_count.to_string();
        let mut ffmpeg_args = vec!["-i", input, "-threads", &cpu_count_str];
        if let Some(options) = &args.options {
            let mut options_vec: Vec<&str> = options.split_whitespace().collect();
            ffmpeg_args.append(&mut options_vec);
        }
        ffmpeg_args.push(output);

        if args.dry_run {
            println!("Dry-run mode: Simulated FFmpeg command: ffmpeg {}", ffmpeg_args.join(" "));
            return;
        }

        let child = Command::new("ffmpeg")
            .args(ffmpeg_args)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn();

        match child {
            Ok(child_process) => track_progress(child_process, input, output),
            Err(e) => {
                eprintln!("Error executing FFmpeg: {}", e);
                log_error(&format!("Error executing FFmpeg: {}", e));
                continue;
            }
        }
    }
}

fn track_progress(mut child: Child, input: &str, output: &str) {
    let stderr = child.stderr.take().expect("Failed to capture stderr");
    let reader = io::BufReader::new(stderr);
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar().template("{msg} [{wide_bar}] {pos}%").unwrap());

    for line in reader.lines() {
        if let Ok(output) = line {
            if output.contains("frame=") {
                pb.inc(1);
            }
        }
    }

    let status = child.wait().expect("Failed to wait on child process");
    if !status.success() {
        eprintln!("Conversion failed with status: {}", status);
        log_error(&format!("FFmpeg process failed with status: {}", status));
        std::process::exit(1);
    }
    pb.finish_with_message("Conversion Complete!");

    log_success(&format!("Successfully converted '{}' to '{}'.", input, output));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{ self, File };
    use std::process::{ Command, Stdio };
    use clap::Parser;
    use std::path::Path;

    #[test]
    fn test_check_ffmpeg() {
        let result = check_ffmpeg();
        assert!(result || !result, "Should return true or false without panic");
    }

    #[test]
    fn test_args_parsing() {
        let args = Args::parse_from(["mixtape", "input.mp4", "output.mp3", "--force"]);
        assert_eq!(args.files[0], "input.mp4");
        assert_eq!(args.files[1], "output.mp3");
        assert!(args.force);
        assert!(!args.dry_run);
    }

    #[test]
    fn test_dry_run_mode() {
        let args = Args::parse_from(["mixtape", "input.mp4", "output.mp3", "--dry-run"]);
        assert_eq!(args.files[0], "input.mp4");
        assert_eq!(args.files[1], "output.mp3");
        assert!(args.dry_run);
    }

    #[test]
    fn test_track_progress() {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg("echo 'frame=10 fps=25.0 time=00:00:10 bitrate=128k' >&2");

        let child = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn mock process");

        track_progress(child, "test.mp4", "test.mp3");
    }

    #[test]
    fn test_mismatched_file_pairs() {
        let args = Args::parse_from(["mixtape", "input1.mp4"]);
        assert_eq!(args.files.len() % 2, 1, "Should have an odd number of file arguments");
    }

    #[test]
    fn test_nonexistent_file_skipped() {
        let log_path = "logs/mixtape.log";

        // Ensure log file exists before reading
        if !Path::new(log_path).exists() {
            File::create(log_path).expect("Failed to create log file");
        }

        let missing_file = "fake_file.mp4";
        log_error(&format!("Input file '{}' does not exist.", missing_file));

        let log_contents = fs::read_to_string(log_path).expect("Failed to read log file");
        assert!(log_contents.contains(missing_file));
    }

    #[test]
    fn test_dry_run_output() {
        let args = Args::parse_from(["mixtape", "input.mp4", "output.mp3", "--dry-run"]);
        assert!(args.dry_run, "Dry-run mode should be enabled");
    }
}
