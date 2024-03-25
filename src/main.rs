use clap::Parser;

use cli::Cli;
use fetch::fetch;

mod fetch;
mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.tool.as_str() {
        "fetch" => {
            fetch(cli);
        },
        _ => println!("Unknown tool")

    }
}
