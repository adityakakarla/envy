use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use std::fs::{read_to_string, write};
use std::path::Path;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run => run_command(),
    }
}

fn run_command() -> Result<()> {
    let dotenv_file = read_dotenv(Path::new("./.env"))?;
    let dotenv_example_file_content = generate_dotenv_example_file(&dotenv_file)?;
    write_dotenv_example(Path::new("./.env.example"), &dotenv_example_file_content)?;

    Ok(())
}

fn read_dotenv(path: &Path) -> Result<String> {
    let dotenv_content = read_to_string(path).context("Failed to read .env file")?;
    Ok(dotenv_content)
}

fn generate_dotenv_example_file(dotenv_file: &String) -> Result<String> {
    let mut dotenv_example_lines: Vec<String> = Vec::new();

    for line in dotenv_file.trim_ascii().lines() {
        if line.trim() == "" || line.starts_with("#") {
            continue;
        }
        let api_key_name = line.split('=').next().context("Parsing .env file lines")?;
        dotenv_example_lines.push(format!("{api_key_name}="));
    }
    dotenv_example_lines.push(String::new());

    Ok(dotenv_example_lines.join("\n"))
}

fn write_dotenv_example(path: &Path, dotenv_example_file_content: &String) -> Result<()> {
    write(path, dotenv_example_file_content).context("Failed to write .env.example file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_dotenv() -> Result<()> {
        let dotenv_file_content = r#"OPENAI_API_KEY=ABC123
GROK_API_KEY=DEF456
"#
        .to_string();

        let dotenv_example_file_content = generate_dotenv_example_file(&dotenv_file_content)?;

        assert_eq!(
            dotenv_example_file_content,
            r#"OPENAI_API_KEY=
GROK_API_KEY=
"#
        );

        Ok(())
    }

    #[test]
    fn without_newline() -> Result<()> {
        let dotenv_file_content = r#"OPENAI_API_KEY=ABC123
GROK_API_KEY=DEF456"#
            .to_string();

        let dotenv_example_file_content = generate_dotenv_example_file(&dotenv_file_content)?;

        assert_eq!(
            dotenv_example_file_content,
            r#"OPENAI_API_KEY=
GROK_API_KEY=
"#
        );
        Ok(())
    }

    #[test]
    fn with_blank_lines() -> Result<()> {
        let dotenv_file_content = r#"
OPENAI_API_KEY=ABC123



GROK_API_KEY=DEF456

"#
        .to_string();

        let dotenv_example_file_content = generate_dotenv_example_file(&dotenv_file_content)?;

        assert_eq!(
            dotenv_example_file_content,
            r#"OPENAI_API_KEY=
GROK_API_KEY=
"#
        );
        Ok(())
    }

    #[test]
    fn with_comments() -> Result<()> {
        let dotenv_file_content = r#"
OPENAI_API_KEY=ABC123
# this is pretty cool
GROK_API_KEY=DEF456
"#
        .to_string();

        let dotenv_example_file_content = generate_dotenv_example_file(&dotenv_file_content)?;

        assert_eq!(
            dotenv_example_file_content,
            r#"OPENAI_API_KEY=
GROK_API_KEY=
"#
        );
        Ok(())
    }
}
