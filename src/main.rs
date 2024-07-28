use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// update command
    #[arg(short, long)]
    update: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.update {
        println!("run update");
    } else {
        println!("run");
    }
}
