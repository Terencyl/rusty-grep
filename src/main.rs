use std::error::Error;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(name = "rg")]
#[command(about = "A simple grep tool allowing to search for a pattern in files")]
#[command(author, version)]
struct Args {
    /// Pattern to search for
    #[arg(value_name = "PATTERN")]
    pattern: String,

    /// Files to search
    #[arg(value_name = "FILE")]
    file: Vec<String>,

    /// Show line numbers with output lines
    #[arg(short = 'n', long = "line-number")]
    show_line_numbers: bool,

    /// Ignore case in patterns and files data
    #[arg(short = 'i', long = "ignore-case")]
    case_insensitive: bool,

    /// Invert the matching so that it selects non-matching lines
    #[arg(short = 'v', long = "invert-match")]
    invert_match: bool,

    /// Output a count of matching lines for each file instead of the normal output
    #[arg(short = 'c', long = "count", conflicts_with_all = ["show_line_numbers", "files_with_matches"])]
    count: bool,

    /// Output the name of each file where matches were found instead of the normal output
    #[arg(short = 'l', long = "files-with-matches", conflicts_with_all = ["show_line_numbers", "count"])]
    files_with_matches: bool,

    /// Matches only lines containing the whole pattern, preceded or followed by non-word characters
    #[arg(short = 'w', long = "word-regexp")]
    whole_words: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}

/// Search for a pattern in files
fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let pattern = get_pattern(&args);
    let regex = get_regex(&args)?;
    let show_filename = args.file.len() > 1;

    for file in &args.file {
        if let Err(e) = process_file(file, &args, &pattern, &regex, show_filename) {
            eprintln!("{}: {}", file, e);
        }
    }
    Ok(())
}

/// Get the search pattern, converting to lowercase if case-insensitive
fn get_pattern(args: &Args) -> String {
    if args.case_insensitive {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    }
}

/// Create regex for whole-word matching if needed
fn get_regex(args: &Args) -> Result<Option<Regex>, Box<dyn Error>> {
    if args.whole_words {
        let pattern = format!(r"\b{}\b", regex::escape(&args.pattern));
        let regex = if args.case_insensitive {
            Regex::new(&format!(r"(?i){}", pattern))?
        } else {
            Regex::new(&pattern)?
        };
        Ok(Some(regex))
    } else {
        Ok(None)
    }
}

/// Process a single file to find matching patterns inside of it and print them
fn process_file(
    file: &str,
    args: &Args,
    pattern: &str,
    regex: &Option<Regex>,
    show_filename: bool,
) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(file)?;
    let mut count = 0;

    for (index, line) in contents.lines().enumerate() {
        if get_matches(args, pattern, line, regex) {
            if args.files_with_matches {
                println!("{file}");
                return Ok(());
            } else if args.count {
                count += 1;
            } else {
                print_match(file, index, line, args.show_line_numbers, show_filename);
            }
        }
    }

    if args.count {
        if show_filename {
            println!("{file}:{count}");
        } else {
            println!("{count}");
        }
    }
    Ok(())
}

/// Check if a line matches the pattern
fn get_matches(args: &Args, pattern: &str, line: &str, regex: &Option<Regex>) -> bool {
    let matches = if let Some(re) = regex {
        re.is_match(line)
    } else if args.case_insensitive {
        line.to_lowercase().contains(pattern)
    } else {
        line.contains(pattern)
    };

    if args.invert_match { !matches } else { matches }
}

/// Print the matches of pattern in file to the output
fn print_match(file: &str, index: usize, line: &str, show_line_number: bool, show_filename: bool) {
    let prefix = if show_filename {
        if show_line_number {
            format!("{file}:{}:", index + 1)
        } else {
            format!("{file}:")
        }
    } else {
        if show_line_number {
            format!("{}:", index + 1)
        } else {
            String::new()
        }
    };

    println!("{prefix}{line}");
}
