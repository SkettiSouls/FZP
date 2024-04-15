use std::{env::set_current_dir, path::PathBuf, process::exit};
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

    /// Directory to search in [default: $HOME/Music]
    path: Option<PathBuf>,
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
    let cli = Cli::parse();
    match cli {
        Cli { directory: Some(args), .. } => {
            // Assigns args to music_directory if a path is provided, and defaults to the xdg audio
            // directory if no path is provided.
            let music_directory = args.unwrap_or_else(|| dirs::audio_dir().expect("Failed to find default music directory."));

            // Change to the provided directory.
            set_current_dir(&music_directory).expect("Failed to change directory.");
        },

        // The compiler demands this arm be included, despite being unreachable.
        // This arm is unreachable because the function `mode_directory()` CANNOT
        // be run if directory is None. It MUST be `directory: Some(_)`.
        Cli { directory: None, .. } => {
            println!("Paradox encountered: Directory mode unset in directory mode.");
            exit(1);
        }
    }
}

fn mode_fuzzy() {
    let cli = Cli::parse();
    match cli {
        Cli { path: Some(music_directory), .. } => {
            set_current_dir(&music_directory).expect("Failed to change directory.");
        },

        Cli { path: None, .. } => {
            let music_directory = dirs::audio_dir().expect("Failed to find default music directory.");

            set_current_dir(&music_directory).expect("Failed to change directory.");
        }
    }
}

fn mode_playlist() {
    let cli = Cli::parse();
    match cli {
        Cli { playlist: Some(args), .. } => {
            let music_directory = args.unwrap_or_else(|| dirs::audio_dir().expect("Failed to find default music directory."));

            set_current_dir(&music_directory).expect("Failed to change directory.");
        },

        Cli { playlist: None, .. } => {
            println!("Paradox encountered: Directory mode unset in directory mode.");
            exit(1);
        }
    }
}
