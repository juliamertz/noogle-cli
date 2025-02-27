mod models;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use models::Docs;
use std::io::{BufRead, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Nix function exploring
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print out list of all functions
    List {
        /// print entries as serialized JSON to stdout
        #[arg(short, long)]
        json: bool,
    },

    /// Fuzzy search function titles
    Search {
        /// query string to match against function titles
        query: String,
    },

    /// Show function metadata
    Show {
        /// print entriy as serialized JSON to stdout
        #[arg(short, long)]
        json: bool,

        /// function path, if none is given it will read from stdin
        path: Option<String>,
    },

    /// Print markdown documentation for given function
    Doc {
        /// function path, if none is given it will read from stdin
        path: Option<String>,
    },
}

const RAW_JSON: &str = include_str!("../data.json");

fn main() -> Result<()> {
    let cli = Cli::parse();
    let docs: Docs = serde_json::from_str(RAW_JSON).unwrap();

    let result = match cli.command {
        Command::List { json } => handle_list_command(json, docs)?,
        Command::Search {
            query,
        } => {

            handle_search_command(&query, docs)
        }
        Command::Show { path, json } => handle_show_command(path, json, docs)?,
        Command::Doc { path } => handle_doc_command(path, docs)?,
    };

    Ok(std::io::stdout().write_all(result.as_bytes())?)
}

/// If `string` is `None` read single line from stdin
fn string_or_stdin(string: Option<String>) -> Result<String> {
    match string {
        Some(value) => Ok(value),
        None => {
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            let mut buf = String::new();
            handle.read_line(&mut buf)?;
            Ok(buf.trim().to_string())
        }
    }
}

fn handle_list_command(json: bool, docs: Docs) -> Result<String> {
    let stdout = if json {
        serde_json::to_string_pretty(&docs)?
    } else {
        docs.0
            .into_iter()
            .map(|func| func.meta.title)
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(stdout)
}

fn handle_search_command(query: &str, docs: Docs) -> String {
    let results = docs.fuzzy_search_sorted(query);
    let mut stdout = String::new();
    for (content, _) in results {
        stdout += &content.meta.title;
        stdout += "\n";
    }

    stdout
}

fn handle_show_command(path: Option<String>, json: bool, docs: Docs) -> Result<String> {
    let title = string_or_stdin(path)?;
    let entry = docs.get_by_title(title).context("no such function")?;

    if json {
        Ok(serde_json::to_string_pretty(entry)?)
    } else {
        Ok(format!("{:?}", entry))
    }
}

fn handle_doc_command(path: Option<String>, docs: Docs) -> Result<String> {
    let title = string_or_stdin(path)?;
    docs.get_by_title(title)
        .context("no such function")?
        .content
        .clone()
        .and_then(|source| source.content)
        .context("function has no documentation")
}
