use std::env;
use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let ffmpeg_src = "bin/ffmpeg"; // Path to our bundled FFmpeg binary
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let ffmpeg_dest = PathBuf::from(&out_dir).join("ffmpeg");

    // Ensure the source binary exists before proceeding
    if !PathBuf::from(ffmpeg_src).exists() {
        eprintln!("cargo:warning=FFmpeg binary not found at {}", ffmpeg_src);
        panic!("FFmpeg binary is missing. Ensure it is placed in the 'bin' directory.");
    }

    // Ensure the output directory exists
    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!("cargo:warning=Failed to create output directory: {}", e);
        panic!("Failed to create output directory");
    }

    // Attempt to copy FFmpeg binary
    if let Err(e) = fs::copy(ffmpeg_src, &ffmpeg_dest) {
        eprintln!("cargo:warning=Failed to copy FFmpeg binary: {}", e);
        panic!("Failed to copy FFmpeg binary");
    }

    // Ensure execute permissions are set
    let mut perms = fs::metadata(&ffmpeg_dest).expect("Failed to get file metadata").permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    fs::set_permissions(&ffmpeg_dest, perms).expect("Failed to set execute permissions on FFmpeg");

    println!("cargo:warning=FFmpeg binary copied successfully to {:?}", ffmpeg_dest);
}
