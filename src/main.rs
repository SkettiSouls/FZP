use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "Fuzzy player", long_about = None)]
struct Cli {
    /// Search by directory
    #[arg(long, short)]
    directory: Option<Option<PathBuf>>,

    /// Enable media looping
    #[arg(long = "loop", short = 'l')]
    does_loop: bool,

    /// Enable multiple selections using TAB
    #[arg(long, short)]
    multi: bool,

    /// Search by playlist/filetype
    #[arg(long, short, conflicts_with = "directory")]
    playlist: Option<Option<PathBuf>>,

    /// File extension to search by in playlist mode
    #[arg(long, short, default_value = "m3u")]
    extension: String,
}

fn main() {
    let cli = Cli::parse();
    match cli {
        // Check if directory mode is set.
        Cli { directory: Some(_), .. } => mode_directory(),
        // Check if playlist mode is set.
        Cli { playlist: Some(_), .. } => mode_playlist(),
        // In all other cases, run fuzzy mode.
        _ => mode_fuzzy()
    }
}

fn mode_directory() {
    // directory logic
}

fn mode_fuzzy() {
    // fuzzy logic
}

fn mode_playlist() {
    // playlist logic
}
