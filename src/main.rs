use std::{env::set_current_dir, path::PathBuf};
use clap::Parser;
use tui::start_tui;

mod tui;

#[derive(Parser, Debug)]
#[command(version, about = "Fuzzy player", long_about = None)]
struct Cli {
    /// Directory to search in
    // Default to $XDG_MUSIC_DIR
    #[arg(default_value = dirs::audio_dir().expect("XDG music directory not found. Please specify a path manually or set XDG_MUSIC_DIR.").into_os_string())]
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
    let music_directory = cli.path.unwrap();

    set_current_dir(music_directory).expect("Failed to change directory.");

    // TODO: Fuzzy finding for music selection

    match cli {
        Cli { directory: true, .. } => mode_directory(),
        Cli { playlist: true, .. } => mode_playlist(),
        _ => mode_fuzzy(),
    };
}

fn mode_fuzzy() {
    let cli = Cli::parse();

    println!("You are in fuzzy mode at path `{:#?}`", cli.path.unwrap());
    start_tui().expect("Failed to start tui");
}

fn mode_directory() {
    let cli = Cli::parse();

    println!("You are in directory mode at path `{:#?}`", cli.path.unwrap());
}

fn mode_playlist() {
    let cli = Cli::parse();

    println!("You are in playlist mode at path `{:#?}`", cli.path.unwrap());
}
