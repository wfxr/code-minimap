mod cli;
mod util;
use cli::{CompletionOpt, Opt, StructOpt, Subcommand};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    util::reset_signal_pipe_handler();
    let opt: Opt = Opt::from_args();
    match &opt.subcommand {
        Some(Subcommand::Completion(CompletionOpt { shell })) => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), *shell, &mut std::io::stdout());
        }
        None => {
            let stdin = io::stdin();
            let reader: Box<dyn BufRead> = match &opt.file {
                Some(path) => Box::new(BufReader::new(File::open(path)?)),
                None => Box::new(stdin.lock()),
            };
            code_minimap::printstd(reader, opt.hscale, opt.vscale, opt.padding)?;
        }
    }
    Ok(())
}
