use clap::Parser;
use jito_reward_cli::cli_args::Args;

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.name);
}
