use std::error::Error;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(name = "rg")]
#[command(about = "A simple grep tool")]
struct Args {
    pattern: String,
    file: Vec<String>,
    #[arg(short = 'n', long)]
    line_number: bool,
    #[arg(short = 'i', long)]
    case_insensitive: bool,
    #[arg(short = 'v', long)]
    invert_matches: bool,
    #[arg(short = 'c', long)]
    count_matches: bool,
    #[arg(short = 'l', long)]
    only_filenames: bool,
    #[arg(short = 'w', long)]
    whole_words: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    };
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let pattern = get_pattern(&args);
    let regex = get_regex(&args)?;

    for file in &args.file {
        let contents = std::fs::read_to_string(file)?;
        let mut count = 0;

        for (index, line) in contents.lines().enumerate() {
            let has_matches = get_matches(&args, &pattern, line, &regex);

            if has_matches {
                if args.only_filenames {
                    println!("{file}");
                    break;
                } else if args.count_matches {
                    count += 1;
                } else {
                    print_matches(&args, file, index, line);
                }
            }
        }

        if args.count_matches {
            println!("{file}:{count}");
        }
    }

    Ok(())
}

fn get_pattern(args: &Args) -> String {
    if args.case_insensitive {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    }
}

fn print_matches(args: &Args, file: &str, index: usize, line: &str) {
    if args.line_number {
        println!("{file}:{}:{line}", index + 1);
    } else {
        println!("{file}:{line}");
    }
}

fn get_matches(args: &Args, pattern: &str, line: &str, regex: &Option<Regex>) -> bool {
    let has_matches = if let Some(re) = regex {
        re.is_match(line)
    } else if args.case_insensitive {
        line.to_lowercase().contains(pattern)
    } else {
        line.contains(pattern)
    };

    if args.invert_matches {
        !has_matches
    } else {
        has_matches
    }
}

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
