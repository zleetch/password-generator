mod generate;

use generate::generate_random_string;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "password-generator")]
#[command(about = "Simple CLI for password generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short = 'L', long, default_value_t = 8)]
        length: usize,

        #[arg(short, long)]
        number: bool,

        #[arg(short, long)]
        symbol: bool,

        #[arg(short, long)]
        uppercase: bool,

        #[arg(short, long)]
        lowercase: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {
            length,
            number,
            symbol,
            uppercase,
            lowercase,
        } => {
            let result = generate_random_string(*length, *number, *symbol, *uppercase, *lowercase);
            println!("Result: {}", result)
        }
    }
}
