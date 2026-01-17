use std::env;
use log_scanner::{read_log, extract_warnings_and_errors, present_output, save_output};
 
fn main() {

    //let args: Vec<String> = env::args().collect();
    //let path = &args[1];
    let path: String = String::from("./example/example2.log");

    let log_file = read_log(&path);
    let messages = extract_warnings_and_errors(&log_file);
    present_output(&messages);
    save_output("test", &messages);
}