use clap::Parser;

/// A simple brainfuck interpreter
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The file path of the brainfuck source code
    #[arg(value_hint=clap::ValueHint::DirPath)]
    pub path: String,

    /// Debug mode that allows debugging brainfuck code
    #[arg(short, long)]
    pub debug: bool,
}
