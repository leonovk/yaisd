use clap::Parser;
mod process_manager;
mod self_update;

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
        if let Err(e) = self_update::update() {
            println!("[ERROR] {}", e);
            ::std::process::exit(1);
        }
    } else {
        process_manager::run();
    }
}
