use std::path::PathBuf;

use clap::{AppSettings, Parser};
use clap_complete::Shell;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct App {
    /// File to read
    pub file: Option<PathBuf>,

    /// Specify horizontal scale factor
    #[clap(short = 'H', long = "horizontal-scale", default_value = "1.0")]
    pub hscale: f64,

    /// Specify vertical scale factor
    #[clap(short = 'V', long = "vertical-scale", default_value = "1.0")]
    pub vscale: f64,

    /// Specify padding width
    #[clap(long)]
    pub padding: Option<usize>,

    /// Specify input encoding
    #[clap(long, default_value = Encoding::VARIANTS[0], possible_values = Encoding::VARIANTS, ignore_case = true)]
    pub encoding: Encoding,

    /// Subcommand
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Parser)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Parser)]
pub struct CompletionOpt {
    /// Target shell name
    #[clap(arg_enum)]
    pub shell: Shell,
}

#[derive(Display, EnumString, EnumVariantNames, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[strum(ascii_case_insensitive)]
pub enum Encoding {
    UTF8Lossy,
    UTF8,
}
