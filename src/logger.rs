use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "logs/mixtape.log";

pub fn log_error(message: &str) {
    log_message("ERROR", message);
}

pub fn log_success(message: &str) {
    log_message("SUCCESS", message);
}

fn log_message(level: &str, message: &str) {
    let now = Local::now();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_FILE)
        .expect("Failed to open log file");

    writeln!(file, "[{}] {}: {}", now.format("%Y-%m-%d %H:%M:%S"), level, message).expect(
        "Failed to write to log file"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_error() {
        let test_message = "This is a test error log";
        log_error(test_message);
        let log_contents = fs::read_to_string(LOG_FILE).expect("Failed to read log file");
        assert!(log_contents.contains(test_message));
    }

    #[test]
    fn test_log_success() {
        let test_message = "This is a test success log";
        log_success(test_message);
        let log_contents = fs::read_to_string(LOG_FILE).expect("Failed to read log file");
        assert!(log_contents.contains(test_message));
    }

    #[test]
    fn test_log_multiple_errors() {
        let test_message_1 = "First error log";
        let test_message_2 = "Second error log";
        log_error(test_message_1);
        log_error(test_message_2);
        let log_contents = fs::read_to_string(LOG_FILE).expect("Failed to read log file");
        assert!(log_contents.contains(test_message_1));
        assert!(log_contents.contains(test_message_2));
    }
}
