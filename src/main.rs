use log_scanner::{extract_messages, present_output, read_file, save_output};
use actix_web::{post, web, App, HttpServer, HttpResponse};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "log_scanner")]
#[command(about = "Extracts messages from log files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the CLI tool
    Cli {
        /// Path to the log file
        log: String,
    },
    /// Start the HTTP server
    Server,
}

#[post("/process")]
async fn process_log(log_data: web::Bytes) -> HttpResponse {
    let log_path = String::from_utf8(log_data.to_vec()).unwrap_or_else(|_| "Invalid UTF-8".to_string());
    let output = scan_log_file(&log_path);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(output)
}

fn run_as_cli(log_data: &String) {
    scan_log_file(log_data);
}

fn scan_log_file(path: &String) {
    let log_file = read_file(path);
    let undesired_notes = read_file("./input/undesired_notes.txt");
    let messages = extract_messages(&log_file, &undesired_notes);
    present_output(&messages);
    save_output("test", &messages);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Cli {log } => {
            run_as_cli(log);
            Ok(())
        }
        Commands::Server => {
            HttpServer::new(|| App::new().service(process_log))
                .bind("0.0.0.0:8080")?
                .run()
                .await
        }
    }
}