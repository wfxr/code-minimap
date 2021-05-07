mod cli;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    process,
};

use cli::{CompletionOpt, Encoding, Opt, StructOpt, Subcommand};
use code_minimap::lossy_reader::LossyReader;

fn main() {
    if let Err(e) = try_main() {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let opt: Opt = Opt::from_args();
    match &opt.subcommand {
        Some(Subcommand::Completion(CompletionOpt { shell })) => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), *shell, &mut std::io::stdout());
        }
        None => {
            let stdin = io::stdin();
            let reader = match &opt.file {
                Some(path) => buf_reader(&opt.encoding, File::open(path)?),
                None => buf_reader(&opt.encoding, stdin),
            };
            code_minimap::print(reader, opt.hscale, opt.vscale, opt.padding)?;
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
