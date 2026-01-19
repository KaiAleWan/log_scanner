use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;

/// Imports the .log or .txt file and store its contents in a String
/// The function automatically handles potential errors triggered
/// by the `read_to_string` function and ensures to always return a
/// String that can be consumed by other functions.
///
/// # Returns
/// `String`: The contents of the input file or an error message
pub fn read_file(file_path: &str) -> String {
    if let Ok(contents) = fs::read_to_string(file_path) {
        contents
    } else {
        String::from("Error: Failed to read the log file")
    }
}

/// Processes the input log file path to derive a name for
/// the output file. 
/// The derived name will be <input file> messages.txt.
///
/// # Returns
/// `String`: The derived name for the output file
pub fn derive_file_name(path: &str) -> String {
    let segments: Vec<&str> = path.split("/").collect();
    format!("{} messages.txt", segments.last().unwrap())
}

/// Converts an input string into a `Regex` struct which is used
/// to detect undesired notes.
/// Note that the function always adds '^' at the start of the 
/// pattern to start scanning from the start of the line.
/// In addition, it replaces the '<X>' with '.*' which is a wild card
/// for 0 or more symbols.
/// 
/// # Arguments
/// * `input` - String slice that will be used to create the `Regex`
/// # Returns
/// `Regex`: Pattern to detect an undesired note in the log file
fn convert_string_to_regex(input: &str) -> Regex {
    let regex_string = format!("^{}", input.replace("<X>", ".*"));
    Regex::new(&regex_string).unwrap()
}

/// Takes a string slice and converts it into a list of `Regex`
/// structs which can be used to identify undesired notes.
/// In addition, the output will always include Regex patterns
/// to detect warnings and errors.
///
/// # Returns
/// `Vec<Regex>`: A vector Regex patterns to detect undesired notes, warnings, and errors
fn prepare_regex_patterns(text: &str) -> Vec<Regex> {
    let input: Vec<&str> = text.split('\n').collect();
    let mut output: Vec<Regex> = vec![];
    if !text.is_empty() {
        for s in input {
            output.push(convert_string_to_regex(s));
        }
    }
    output.push(Regex::new(r"^WARNING:").unwrap()); // Pattern to detect warnings
    output.push(Regex::new(r"^ERROR:").unwrap()); // Pattern to detect errors    
    output
}

/// Checks whether a messages matches any of the `Regex` patterns for
/// warnings, errors, or undesired notes.
///
/// # Arguments
/// * `input` - String slice that will be examined
/// * `patterns` - Reference to a vector storing `Regex` patterns
///
/// # Returns
/// `bool`: Is `true` if `input` matches one of the submitted `Regex` patterns
fn match_regex_pattern(input: &str, patterns: &Vec<Regex>) -> bool {
    let mut result = false;
    for pattern in patterns {
        if pattern.is_match(input) {
            result = true;
        }
    }
    result
}

/// Splits a `String` by its line breaks and checks if a line contains
/// a warning or an error. If so, the `&str` will be converted
/// into a new `String` and stored in the output vector.
///
/// # Returns
/// `Vec<String>`: A vector with warning and error messages
pub fn extract_messages(text: &str, undesired_notes: &str) -> Vec<String> {
    let input: Vec<&str> = text.split('\n').collect();
    let regex_patterns = prepare_regex_patterns(undesired_notes);
    let mut output: Vec<String> = vec![];
    for s in input {
        if match_regex_pattern(s, &regex_patterns) {
            output.push(s.to_string());
        }
    }
    output
}

/// Prints the contents of a Vec<String> to the console.
pub fn present_output(messages: &Vec<String>) {
    if !messages.is_empty() {
        for message in messages {
            println!("{}", message);
        }
    } else {
        println!("No issues were detected in the log file.");
    }
}

/// Saves the contents of a Vec<String> to a file located in
/// the ./output folder.
/// If the vector is empty, it will create a generic message instead.
///
/// # Panics
/// The function panics if the output file cannot be created or updated.
pub fn save_output(filename: &str, messages: &Vec<String>) {
    let file_path = format!("./output/{}", filename);
    let mut file = File::create(file_path).unwrap();
    if !messages.is_empty() {
        for message in messages {
            file.write_all(message.as_bytes()).unwrap();
        }
    } else {
        file.write_all(b"No issues were found in the log file.")
            .unwrap();
    }
}

// Section for unit tests

#[cfg(test)]
mod tests {
    use super::*;

    // Check if the read_log function is able to extract the contents of a .log file
    #[test]
    fn it_reads_log() {
        let result = read_file("./example/example1.log");
        println!("{}", result);
        assert_eq!(result, String::from("Import was successful."));
    }

    // Check if the output filename is derived correctly
    #[test]
    fn it_derives_the_filename() {
        let path = "./example/example1.log";
        let result = derive_file_name(path);
        assert_eq!(result, "example1.log messages.txt");
    }

    // Check if the extract_warnings_and_errors function is able to identify and extract warnings and errors
    #[test]
    fn it_extracts_messages() {
        let test_string = String::from(
            "WARNING: a\nb\nERROR: c\nFalse WARNING: should not extract this one\nNOTE: Test note d",
        );
        let result = extract_messages(&test_string, "NOTE: Test note <X>");
        assert_eq!(result, ["WARNING: a", "ERROR: c", "NOTE: Test note d"]);
    }
}
