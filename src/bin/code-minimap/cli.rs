use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};
use clap_complete::Shell;

#[derive(Parser)]
#[command(about, version)]
#[command(next_line_help = true)]
#[command(disable_version_flag = true)]
pub struct App {
    /// File to read.
    pub file: Option<PathBuf>,

    /// Specify horizontal scale factor.
    #[arg(short = 'H', long = "horizontal-scale", default_value = "1.0")]
    pub hscale: f64,

    /// Specify vertical scale factor.
    #[arg(short = 'V', long = "vertical-scale", default_value = "1.0")]
    pub vscale: f64,

    #[arg(long)]
    /// Specify padding width.
    pub padding: Option<usize>,

    /// Specify input encoding.
    #[arg(long, value_enum, default_value_t = Encoding::UTF8Lossy, ignore_case = true)]
    pub encoding: Encoding,

    /// Subcommand.
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    /// Print version.
    #[arg(long, action = ArgAction::Version)]
    version: Option<bool>,
}

#[derive(Parser)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion {
        /// Target shell name.
        shell: Shell,
    },
}

#[derive(Parser)]
pub struct CompletionOpt {
    /// Target shell name.
    pub shell: Shell,
}

#[derive(Copy, Clone, ValueEnum)]
pub enum Encoding {
    UTF8Lossy,
    UTF8,
}
