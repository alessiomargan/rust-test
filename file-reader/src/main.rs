use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// The path to the file to read
    #[arg(short, long)]
    path: std::path::PathBuf,
}


fn greet(name: &str, count: u8) {
    for _ in 0..count {
        println!("Hello {}!", name);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    match std::fs::read_to_string(filename) {
        Ok(contents) => contents
            .lines()
            .map(String::from)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn main() {

    let args = Args::parse();

    greet(&args.name, args.count);

    // Check if the path exists
    if !args.path.exists() {
        eprintln!("Error: File '{}' does not exist", args.path.display());
        return;
    }

    // Convert PathBuf to &str - to_str() returns Option<&str>
    match args.path.to_str() {
        Some(path_str) => {
            let lines = read_lines(path_str);
            for line in lines {
                println!("File line: {}", line);
            }
        }
        None => {
            eprintln!("Error: Path contains invalid UTF-8");
        }
    }
}

