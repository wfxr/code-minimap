mod cli;

use anyhow::bail;
use clap::{CommandFactory, Parser};
use cli::{App, CliRenderMode, Encoding, Subcommand};
use code_minimap::{RenderMode, lossy_reader::LossyReader};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, IsTerminal, Read},
    process,
};

fn main() {
    if let Err(e) = try_main() {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>()
            && ioerr.kind() == io::ErrorKind::BrokenPipe
        {
            std::process::exit(0);
        }
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let opt: App = App::parse();
    match opt.subcommand {
        Some(Subcommand::Completion { shell }) => {
            let cmd = &mut App::command();
            clap_complete::generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout())
        }
        None => {
            let stdin = io::stdin();
            let reader = match &opt.file {
                Some(path) => buf_reader(&opt.encoding, File::open(path)?),
                None if std::io::stdin().is_terminal() => bail!("no input file specified (use -h for help)"),
                None => buf_reader(&opt.encoding, stdin),
            };
            let mode = match opt.mode {
                CliRenderMode::Braille => RenderMode::Braille,
                CliRenderMode::Block => RenderMode::Block,
            };
            code_minimap::print(reader, opt.hscale, opt.vscale, opt.padding, mode)?;
        }
    }
    Ok(())
}

fn buf_reader<R: 'static + Read>(encoding: &Encoding, reader: R) -> Box<dyn BufRead> {
    match encoding {
        Encoding::UTF8 => Box::new(BufReader::new(reader)),
        Encoding::UTF8Lossy => Box::new(LossyReader::new(reader)),
    }
}
