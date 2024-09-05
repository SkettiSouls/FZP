use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Fuzzy player", long_about = None)]
struct Cli {
    /// Directory to search in
    #[arg(default_value = "$HOME/Music")]
    path: Option<PathBuf>,

    /// Search by directory
    #[arg(long, short)]
    directory: bool,

    /// Loop selection
    #[arg(long = "loop", short = 'l')]
    does_loop: bool,

    /// Enable multiple selections using TAB
    #[arg(long, short)]
    multi: bool,

    /// Search by playlist/filetype
    #[arg(long, short, conflicts_with = "directory")]
    // TODO: Make an arg for filetype that only exists when playlist = true
    playlist: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli { directory: true, .. } => mode_directory(),
        Cli { playlist: true, .. } => mode_playlist(),
        _ => mode_fuzzy(),
    };
}

fn mode_fuzzy() {
    let cli = Cli::parse();

    println!("You are in fuzzy mode at path `{:#?}`", cli.path.unwrap());
}

fn mode_directory() {
    let cli = Cli::parse();

    println!("You are in directory mode at path `{:#?}`", cli.path.unwrap());
}

fn mode_playlist() {
    let cli = Cli::parse();

    println!("You are in playlist mode at path `{:#?}`", cli.path.unwrap());
}
