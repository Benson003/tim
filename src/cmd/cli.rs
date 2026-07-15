use std::{env, process};

pub const MAJOR: u8 = 0;
pub const MINOR: u8 = 0;
pub const PATCH: u8 = 1;

pub struct CliConfig {
    pub input_file: String,
    pub output_file: Option<String>,
    pub wrap_tag: Option<String>,
    pub dry_run: bool,
}

impl CliConfig {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();

        // 1. Check for basic terminal flags
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            Self::print_help();
            process::exit(0);
        }

        if args.contains(&"--version".to_string()) {
            println!("tim compiler v{}.{}.{}", MAJOR, MINOR, PATCH);
            process::exit(0);
        }

        // 2. We need at least an input file (usually index 1 if no flags are prefixed)
        if args.len() < 2 {
            eprintln!("Error: No input file provided.");
            Self::print_help();
            process::exit(1);
        }

        let mut input_file = None;
        let mut output_file = None;
        let mut wrap_tag = None;
        let mut dry_run = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-o" | "--output-file" => {
                    if i + 1 < args.len() {
                        output_file = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        eprintln!("Error: Missing value for output file.");
                        process::exit(1);
                    }
                }
                "-w" | "--wrap" => {
                    if i + 1 < args.len() {
                        wrap_tag = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        eprintln!("Error: Missing value for wrap tag.");
                        process::exit(1);
                    }
                }
                "--dry-run" => {
                    dry_run = true;
                    i += 1;
                }
                // Anything that doesn't start with '-' is assumed to be the input file
                other if !other.starts_with('-') => {
                    input_file = Some(other.to_string());
                    i += 1;
                }
                unknown => {
                    eprintln!("Error: Unknown flag '{}'", unknown);
                    Self::print_help();
                    process::exit(1);
                }
            }
        }

        let input_path = match input_file {
            Some(file) => file,
            None => {
                eprintln!("Error: No input file specified.");
                process::exit(1);
            }
        };

        if !input_path.ends_with(".tim") {
            eprintln!("Error: 'tim' only compiles files with a '.tim' extension.");
            eprintln!("Found: '{}'", input_path);
            process::exit(1);
        }

        let final_output = if dry_run {
            None // No output file needed for terminal dumps
        } else {
            match output_file {
                Some(path) => Some(path),
                None => {
                    // Slices off ".tim" (last 4 characters) and appends ".html"
                    let base_name = &input_path[..input_path.len() - 4];
                    let inferred = format!("{}.html", base_name);
                    Some(inferred)
                }
            }
        };

        CliConfig {
            input_file: input_path,
            output_file: final_output,
            wrap_tag,
            dry_run,
        }
    }

    fn print_help() {
        println!(
            r#"tim - A lightweight styled markup language compiler

USAGE:
    tim [FLAGS] [OPTIONS] <input_file>

FLAGS:
    -h, --help       Prints help information
    --version        Prints version information
    --dry-run        Dumps the parser AST to terminal without compiling HTML

OPTIONS:
    -o, --output-file <file>   Saves output directly to a file
    -w, --wrap <tag>           Wraps the entire output in a custom HTML tag (e.g., article, body, main)
"#
        );
    }
}
