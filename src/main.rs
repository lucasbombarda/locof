use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(default_value = ".")]
    path: String,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
