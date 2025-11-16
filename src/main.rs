use clap::{Parser, Subcommand};
use std::fs::{read_to_string, write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run {} => {
            let dotenv_file = read_to_string("./.env").unwrap();

            let mut dotenv_example_lines: Vec<String> = Vec::new();

            for line in dotenv_file.trim_ascii().split_ascii_whitespace() {
                let api_key_name = line.split('=').next().unwrap();
                dotenv_example_lines.push(format!("{api_key_name}="));
            }
            dotenv_example_lines.push(String::new());

            let dotenv_example_file_content = dotenv_example_lines.join("\n");
            write("./.env.example", dotenv_example_file_content).unwrap();
        }
    }
}
