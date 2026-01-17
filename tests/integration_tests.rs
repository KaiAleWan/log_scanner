use log_scanner::{read_log, extract_warnings_and_errors};

#[test]
fn can_extract_warnings_and_errors() {
    let path: String = String::from("./example/example2.log");
    let log_file = read_log(&path);
    let messages = extract_warnings_and_errors(&log_file);
    assert_eq!(messages, ["WARNING: Multiple lenghts detected. \r", "ERROR: The expected file xxx does not exist.\r"]);
}