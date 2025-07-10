use std::collections::HashMap;

/// Handles setting exit status codes in REPL variables
pub struct StatusHandler;

impl StatusHandler {
    /// Set the exit status in the REPL variables
    pub fn set_exit_status(repl_vars: &mut HashMap<String, String>, success: bool) {
        let status = if success { "0" } else { "1" };
        repl_vars.insert("?".to_string(), status.to_string());
    }
}

pub struct StdoutHandler;

impl StdoutHandler {
    pub fn print_and_set_last_result(repl_vars: &mut HashMap<String, String>, result: String) {
        println!("{}", result);
        repl_vars.insert("0".to_string(), result);
    }
}

pub fn extract_hostname(url: &str) -> String {
    let url = url.trim();
    let url = url.trim_start_matches("http://");
    let url = url.trim_start_matches("https://");

    // Find the first occurrence of '/', '?', or '#' to get just the hostname
    let hostname = if let Some(pos) = url.find(|c| c == '/' || c == '?' || c == '#') {
        &url[..pos]
    } else {
        url
    };

    // Remove trailing slash if present
    let hostname = hostname.trim_end_matches('/');

    hostname.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hostname() {
        assert_eq!(extract_hostname("https://google.com"), "google.com");
        assert_eq!(extract_hostname("https://google.com/"), "google.com");
        assert_eq!(extract_hostname("https://google.com/test"), "google.com");
        assert_eq!(extract_hostname("https://google.com/test/"), "google.com");
        assert_eq!(
            extract_hostname("https://google.com/test/test"),
            "google.com"
        );
        assert_eq!(
            extract_hostname("https://google.com/test/test/"),
            "google.com"
        );
        assert_eq!(
            extract_hostname("https://google.com/test/test/test"),
            "google.com"
        );
        assert_eq!(
            extract_hostname("https://google.com/test/test/test/"),
            "google.com"
        );
        assert_eq!(
            extract_hostname("https://google.com/test/test/test/test"),
            "google.com"
        );
        assert_eq!(
            extract_hostname("https://google.com?test=test"),
            "google.com"
        );
        assert_eq!(extract_hostname("https://google.com#test"), "google.com");
        assert_eq!(extract_hostname("https://192.168.1.10"), "192.168.1.10");
        assert_eq!(extract_hostname("https://192.168.1.10/"), "192.168.1.10");
        assert_eq!(
            extract_hostname("https://192.168.1.10/test"),
            "192.168.1.10"
        );
        assert_eq!(
            extract_hostname("https://192.168.1.10/test/"),
            "192.168.1.10"
        );
        assert_eq!(
            extract_hostname("https://192.168.1.10/test/"),
            "192.168.1.10"
        );
        assert_eq!(
            extract_hostname("https://192.168.1.10?test=test"),
            "192.168.1.10"
        );
        assert_eq!(
            extract_hostname("https://192.168.1.10#test"),
            "192.168.1.10"
        );
    }
}
