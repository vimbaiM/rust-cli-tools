use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "echor")]
#[command(version = "0.1.0")]
#[command(about = "Rust version of echo")]
#[command(author = "Vimbai")]
/// Rust version of `echo`
struct Echor {
    /// Input text
    #[arg(value_name = "TEXT", required=true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n', long)]
    omit_newline: bool,
}

fn main() {
    let echor = Echor::parse();

    print!("{}{}", echor.text.join(" "), if echor.omit_newline { "" } else { "\n" });
}
