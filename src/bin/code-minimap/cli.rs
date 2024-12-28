use std::path::PathBuf;

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    ArgAction,
    Parser,
    ValueEnum,
};

use clap_complete::Shell;

#[derive(Parser)]
#[clap(about, version)]
#[clap(next_line_help = true)]
#[clap(disable_version_flag = true)]
#[clap(
    styles(Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
    )
)]
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
