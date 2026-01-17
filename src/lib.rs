use std::fs;
use std::fs::File;
use std::io::Write;

pub fn read_log(file_path: &str) -> String {
    if let Ok(contents) = fs::read_to_string(file_path) {
        contents
    } else {
        String::from("Error: Failed to read the log file")
    }
}

pub fn extract_warnings_and_errors(text: &String) ->  Vec<String> {
    let input: Vec<&str> = text.split('\n').collect();
    let mut output: Vec<String> = vec![];
    for s in input {
        if s.contains("WARNING:") || s.contains("ERROR:") {
            output.push(s.to_string());
        } 
    }
    output
}

pub fn present_output(messages: &Vec<String>) {
    if messages.len() > 0 {
        for message in messages {
            println!("{}", message);
        }
    } else {
        println!("No issues were detected in the log file.");
    }
}

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

    #[test]
    fn it_reads_log() {
        let result = read_log("./example/example1.log");
        println!("{}",result);
        assert_eq!(result, String::from("Import was successful."));
    }

    #[test]
    fn it_extracts_warning_and_errors() {
        let test_string = String::from("WARNING: a\nb\nERROR: c");
        let result = extract_warnings_and_errors(&test_string);
        assert_eq!(result, ["WARNING: a", "ERROR: c"]);
    }

}




