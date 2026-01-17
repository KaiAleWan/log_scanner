use std::fs;
use std::fs::File;
use std::io::Write;

/// Imports the .log file and store its contents in a String
/// The function automatically handles potential errors triggered
/// by the `read_to_string` function and ensures to always return a 
/// String that can be consumed by other functions.
/// 
/// # Returns
/// `String`: The contents of the input file or an error message
pub fn read_log(file_path: &str) -> String {
    if let Ok(contents) = fs::read_to_string(file_path) {
        contents
    } else {
        String::from("Error: Failed to read the log file")
    }
}

/// Splits a `String` by its line breaks and checks if a line contains
/// a warning or an error. If so, the `&str` will be converted
/// into a new `String` and stored in the output vector. 
/// 
/// # Returns
/// `Vec<String>`: A vector with warning and error messages
pub fn extract_warnings_and_errors(text: &String) ->  Vec<String> {
    let input: Vec<&str> = text.split('\n').collect();
    let mut output: Vec<String> = vec![];
    for s in input {
        // Needs to be replaced with a more specific method
        if s.contains("WARNING:") || s.contains("ERROR:") {
            output.push(s.to_string());
        } 
    }
    output
}

/// Prints the contents of a Vec<String> to the console. 
pub fn present_output(messages: &Vec<String>) {
    if messages.len() > 0 {
        for message in messages {
            println!("{}", message);
        }
    } else {
        println!("No issues were detected in the log file.");
    }
}

/// Saves the contents of a Vec<String> to a .txt file located in
/// the ./output folder.
/// If the vector is empty, it will create a generic message instead.
/// 
/// # Panics
/// The function panics if the output file cannot be created or updated.
pub fn save_output(filename: &str, messages: &Vec<String>) {
    let file_path = format!("./output/{}.txt", filename);
    let mut file = File::create(file_path).unwrap();
    if messages.len() > 0 {
        for message in messages {
            file.write(message.as_bytes()).unwrap();
        }
    } else {
       file.write(b"No issues were found in the log file.").unwrap(); 
    }
}

// Section for unit tests

#[cfg(test)]
mod tests {
    use super::*;

    // Check if the read_log function is able to extract the contents of a .log file
    #[test]
    fn it_reads_log() {
        let result = read_log("./example/example1.log");
        println!("{}",result);
        assert_eq!(result, String::from("Import was successful."));
    }

    // Check if the extract_warnings_and_errors function is able to identify and extract warnings and errors
    #[test]
    fn it_extracts_warning_and_errors() {
        let test_string = String::from("WARNING: a\nb\nERROR: c");
        let result = extract_warnings_and_errors(&test_string);
        assert_eq!(result, ["WARNING: a", "ERROR: c"]);
    }

}




