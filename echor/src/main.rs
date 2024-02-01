use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(required = true, help = "Input text")]
    text: Vec<String>,

    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    print!("{}{}", args.text.join(" "), if args.omit_newline { "" } else { "\n" })
}
