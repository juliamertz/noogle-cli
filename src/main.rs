mod models;
use models::Doc;

use argh::FromArgs;
use rust_fuzzy_search::fuzzy_compare;
use std::io::{BufRead, Error, ErrorKind, Result, Write};

#[derive(FromArgs, PartialEq, Debug)]
/// Nix function exploring
struct Cli {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    List(ListCommand),
    Search(SearchCommand),
    Show(ShowCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Print out list of all functions
#[argh(subcommand, name = "list")]
struct ListCommand {
    /// print entries as serialized JSON to stdout
    #[argh(switch)]
    json: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Fuzzy search function titles
#[argh(subcommand, name = "search")]
struct SearchCommand {
    /// minimum match score for fuzzy matching 
    #[argh(option, short = 't')]
    threshold: Option<f32>,

    /// query string to match against function titles
    #[argh(positional)]
    query: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Show function metadata
#[argh(subcommand, name = "show")]
struct ShowCommand {
    /// print entry as serialized JSON to stdout
    #[argh(switch)]
    json: bool,

    /// function path
    #[argh(positional)]
    path: Option<String>,
}

const RAW_JSON: &str = include_str!("../data.json");

fn main() -> Result<()> {
    let cli: Cli = argh::from_env();

    let data: Vec<Doc> = serde_json::from_str(RAW_JSON).unwrap();
    let result = match &cli.command {
        Command::List(args) => handle_list_command(args, data)?,
        Command::Search(args) => handle_search_command(args, data),
        Command::Show(args) => handle_show_command(args, &data)?,
    };

    std::io::stdout().write_all(result.as_bytes())
}

fn handle_search_command(args: &SearchCommand, data: Vec<Doc>) -> String {
    let mut matches = data
        .iter()
        .filter_map(|entry| {
            let title = entry.meta.title.as_ref();
            let score = fuzzy_compare(&args.query, title);
            if score >= args.threshold.unwrap_or(0.3) {
                Some((title, score))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    matches.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    let mut stdout = String::new();
    for (content, _) in matches {
        stdout += content;
        stdout += "\n";
    }

    stdout
}

fn handle_show_command(args: &ShowCommand, data: &[Doc]) -> Result<String> {
    let title = match args.path.to_owned() {
        Some(value) => value,
        None => {
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            let mut buf = String::new();
            handle.read_line(&mut buf).unwrap();
            buf.trim().to_string()
        }
    };

    match data.iter().find(|e| e.meta.title == title) {
        Some(entry) => {
            if args.json {
                let serialized = serde_json::to_string_pretty(entry).unwrap();
                return Ok(serialized);
            }

            // TODO: print formatted
            Ok(format!("{:?}", entry))
        }

        None => Err(Error::new(ErrorKind::Other, "No such function found")),
    }
}

fn handle_list_command(args: &ListCommand, data: Vec<Doc>) -> Result<String> {
    let stdout = if args.json {
        serde_json::to_string_pretty(&data)?
    } else {
        data.into_iter()
            .map(|func| func.meta.title)
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(stdout)
}
