use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process;

struct Config {
    show_lines: bool,
    show_words: bool,
    show_chars: bool,
    filename: Option<String>,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        let mut config = Config {
            show_lines: false,
            show_words: false,
            show_chars: false,
            filename: None,
        };

        let mut found_option = false;
        let mut reading_options = true;

        for arg in args.iter().skip(1) {
            if reading_options && arg.starts_with('-') {
                found_option = true;
                for c in arg.chars().skip(1) {
                    match c {
                        'l' => config.show_lines = true,
                        'w' => config.show_words = true,
                        'c' => config.show_chars = true,
                        _ => return Err("Invalid option"),
                    }
                }
            } else {
                reading_options = false;
                config.filename = Some(arg.clone());
            }
        }

        if !found_option {
            config.show_lines = true;
            config.show_words = true;
            config.show_chars = true;
        }

        Ok(config)
    }
}

struct Counts {
    lines: usize,
    words: usize,
    chars: usize,
}

fn count_content<R: Read>(mut reader: BufReader<R>) -> io::Result<Counts> {
    let mut counts = Counts {
        lines: 0,
        words: 0,
        chars: 0,
    };

    let mut byte_buf = Vec::new();
    reader.read_to_end(&mut byte_buf)?;
    counts.chars = byte_buf.len();

    for line in String::from_utf8_lossy(&byte_buf).lines() {
        counts.lines += 1;
        counts.words += line.split_whitespace().count();
    }

    Ok(counts)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    let reader: Box<dyn Read> = match &config.filename {
        Some(filename) => {
            let file = File::open(filename).unwrap_or_else(|err| {
                eprintln!("Error opening file: {}", err);
                process::exit(1);
            });
            Box::new(file)
        }
        None => Box::new(io::stdin()),
    };

    let buf_reader = BufReader::new(reader);

    match count_content(buf_reader) {
        // FIXME: formatting for this is fucking terrible
        Ok(counts) => {
            let mut output = String::new();
            
            if config.show_lines {
                output.push(' ');
                output.push_str(&counts.lines.to_string());
                output.push(' ');
            }
            if config.show_words {
                output.push_str(&counts.words.to_string());
                output.push(' ');
            }
            if config.show_chars {
                output.push_str(&counts.chars.to_string());
                output.push(' ');
            }

            if let Some(filename) = config.filename {
                output.push_str(&filename);
            } else {
                output.push_str("-");
            }

            println!("{}", output);
        }
        Err(err) => {
            eprintln!("Error reading content: {}", err);
            process::exit(1);
        }
    }
}
