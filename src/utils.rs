use std::path::PathBuf;

pub fn is_valid_format(file: &str) -> bool {
    let valid_extensions = ["mp4", "mp3", "avi", "mkv", "mov", "wav", "flac", "jpg", "png", "webp"];
    if let Some(extension) = PathBuf::from(file).extension() {
        return valid_extensions.contains(&extension.to_string_lossy().as_ref());
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_is_valid_format() {
        assert!(is_valid_format("video.mp4"));
        assert!(is_valid_format("audio.mp3"));
        assert!(!is_valid_format("document.txt"));
        assert!(is_valid_format("image.png"));
        assert!(!is_valid_format("unknown.xyz"));
        assert!(is_valid_format("video.final.mp4"));
        assert!(!is_valid_format(".hiddenfile"));
    }

    #[test]
    fn test_invalid_input_format() {
        assert!(!is_valid_format("invalidfile.docx"));
        assert!(!is_valid_format("script.sh"));
    }

    #[test]
    fn test_invalid_output_format() {
        assert!(!is_valid_format("wrongformat.exe"));
        assert!(!is_valid_format("random.bat"));
    }

    #[test]
    fn test_missing_input_file() {
        let missing_file = "nonexistent.mp4";
        assert!(!Path::new(missing_file).exists());
    }
}
