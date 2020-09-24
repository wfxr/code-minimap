use std::path::PathBuf;

use structopt::clap::{self, AppSettings};
pub use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    global_settings(&[AppSettings::ColoredHelp]),
    about = env!("CARGO_PKG_DESCRIPTION"))
]
pub struct Opt {
    /// File to read
    #[structopt(name = "FILE")]
    pub file: Option<PathBuf>,

    /// Specify horizontal scale factor
    #[structopt(short = "H", long = "horizontal-scale", default_value = "1.0")]
    pub hscale: f64,

    /// Specify vertical scale factor
    #[structopt(short = "V", long = "vertical-scale", default_value = "1.0")]
    pub vscale: f64,

    /// Subcommand
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(StructOpt)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion(CompletionOpt),
}

#[derive(StructOpt)]
pub struct CompletionOpt {
    /// Target shell name
    #[structopt(possible_values = &clap::Shell::variants())]
    pub shell: clap::Shell,
}
