use std::env;
use std::fs::{ self, File };
use std::io::Write;
use std::path::Path;
use std::process::{ Command, Output };
use std::str;
use uuid::Uuid;
use regex::Regex;

/// Helper function to get the correct Mixtape binary path.
fn get_binary_path() -> String {
    if let Ok(path) = env::var("CARGO_BIN_EXE_mixtape") {
        return path;
    }

    let mut path = env::current_dir().expect("Failed to get current directory");
    path.push("target/debug/mixtape");

    if !path.exists() {
        panic!("Mixtape executable not found at {:?}. Did you build the project?", path);
    }

    path.to_string_lossy().to_string()
}

/// Helper function to run the Mixtape command with given arguments.
fn run_mixtape(args: &[&str]) -> Output {
    let binary_path = get_binary_path();
    Command::new(binary_path).args(args).output().expect("Failed to execute Mixtape")
}

/// Creates a temporary test file with a unique name.
fn create_test_file(name: &str, extension: &str, content: &[u8]) -> String {
    let filename = format!("test_{}_{}.{}", name, Uuid::new_v4(), extension);
    let mut file = File::create(&filename).expect("Failed to create test file");
    file.write_all(content).expect("Failed to write to test file");
    filename
}

/// Deletes a test file if it exists.
fn cleanup_file(filename: &str) {
    if Path::new(filename).exists() {
        if let Err(e) = fs::remove_file(filename) {
            eprintln!("Warning: Failed to delete test file {}: {}", filename, e);
        } else {
            println!("Deleted test file: {}", filename);
        }
    }
}

/// Deletes all temporary test files after test execution.
fn cleanup_test_files() {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    for entry in fs::read_dir(&current_dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if
                file_name_str.starts_with("test_") &&
                (file_name_str.ends_with(".mp4") || file_name_str.ends_with(".mp3"))
            {
                let file_path = entry.path();
                if let Err(e) = fs::remove_file(&file_path) {
                    eprintln!("Warning: Failed to delete test file {}: {}", file_name_str, e);
                } else {
                    println!("Deleted test file: {}", file_name_str);
                }
            }
        }
    }
}

/// Run cleanup before and after tests to ensure a clean state.
#[test]
fn test_cleanup() {
    cleanup_test_files();
}

/// Tests the `--help` command.
#[test]
fn test_help_command() {
    let output = run_mixtape(&["--help"]);

    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    assert!(stdout.contains("Mixtape is a command-line media conversion tool"));
    assert!(stdout.contains("Usage: mixtape [OPTIONS] <FILES>..."));
    assert!(stdout.contains("--force"));
    assert!(stdout.contains("--dry-run"));
    assert!(stdout.contains("--options"));
    assert!(stdout.contains("--help"));
    assert!(stdout.contains("--version"));
}

/// Tests the `--version` command.
#[test]
fn test_version_command() {
    let output = run_mixtape(&["--version"]);

    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Mixtape"));

    let version_regex = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    assert!(version_regex.is_match(stdout), "Version number format is incorrect");
}

/// Tests behavior when FFmpeg is missing.
#[test]
fn test_missing_ffmpeg() {
    let test_file = create_test_file("input", "mp4", &[0, 1, 2, 3]);
    let output_file = format!("{}.out", test_file);

    let log_path = "logs/mixtape.log";
    if !Path::new(log_path).exists() {
        File::create(log_path).expect("Failed to create log file");
    }

    let output = run_mixtape(&[&test_file, &output_file]);

    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");

    if !output.status.success() && stderr.contains("FFmpeg is not installed") {
        let log_contents = fs::read_to_string(log_path).expect("Failed to read log file");
        assert!(log_contents.contains("FFmpeg is not installed"));
    }

    cleanup_file(&test_file);
}

/// Tests unsupported file formats.
#[test]
fn test_invalid_file_format() {
    let test_file = create_test_file("invalid", "txt", b"This is not a valid media file");
    let output_file = "output.xyz";

    let output = run_mixtape(&[&test_file, output_file]);

    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Unsupported file format"));

    let log_contents = fs::read_to_string("logs/mixtape.log").expect("Failed to read log file");
    assert!(log_contents.contains("Unsupported file format detected"));

    cleanup_file(&test_file);
}

/// Tests dry-run mode.
#[test]
fn test_dry_run_mode() {
    let test_file = create_test_file("dryrun", "mp4", &[0, 1, 2, 3]);
    let output_file = "output.mp3";

    let output = run_mixtape(&[&test_file, output_file, "--dry-run"]);
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    assert!(
        stdout.contains("Dry-run mode: Simulated FFmpeg command"),
        "Dry-run mode should be enabled"
    );
    assert!(!Path::new(output_file).exists());

    cleanup_file(&test_file);
}

/// Tests passing custom FFmpeg options.
#[test]
fn test_custom_options() {
    let output = run_mixtape(&["input.mp4", "output.mp3", "--options", "-b:a 192k", "--dry-run"]);
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    assert!(
        stdout.contains("Dry-run mode: Simulated FFmpeg command"),
        "Dry-run mode should be enabled"
    );
    assert!(stdout.contains("-b:a 192k"), "Custom FFmpeg options should appear in the output");
}

/// Tests `--force` flag.
#[test]
fn test_force_flag() {
    let test_file = create_test_file("force", "mp4", &[0, 1, 2, 3]);
    let output_file = "output_force.mp3";

    let output = run_mixtape(&[&test_file, output_file, "--force", "--dry-run"]);

    assert!(output.status.success());

    cleanup_file(&test_file);
}

/// Tests batch processing of multiple files.
#[test]
fn test_batch_processing() {
    let test_file1 = create_test_file("batch1", "mp4", &[0, 1, 2, 3]);
    let output_file1 = "output_batch1.mp3";
    let test_file2 = create_test_file("batch2", "mp4", &[4, 5, 6, 7]);
    let output_file2 = "output_batch2.mp3";

    let output = run_mixtape(&[&test_file1, output_file1, &test_file2, output_file2, "--dry-run"]);
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    assert!(stdout.contains(&test_file1), "Batch processing should include file1");
    assert!(stdout.contains(&test_file2), "Batch processing should include file2");

    cleanup_file(&test_file1);
    cleanup_file(&test_file2);
}
