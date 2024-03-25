use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub tool: String,

    #[arg(long, short)]
    pub verbose: bool,

    varargs: Vec<String>
}
