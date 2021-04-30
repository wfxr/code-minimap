mod cli;
use std::{
    io::{self, BufRead},
    process,
};

use cli::{CompletionOpt, Opt, StructOpt, Subcommand};
use code_minimap::LossyReader;

fn main() {
    if let Err(e) = try_main()
    {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>()
        {
            if ioerr.kind() == io::ErrorKind::BrokenPipe
            {
                std::process::exit(0);
            }
        }
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let opt: Opt = Opt::from_args();
    match &opt.subcommand
    {
        Some(Subcommand::Completion(CompletionOpt {
            shell,
        })) =>
        {
            Opt::clap().gen_completions_to(
                env!("CARGO_PKG_NAME"),
                *shell,
                &mut std::io::stdout(),
            );
        },
        None =>
        {
            let stdin = io::stdin();
            let reader: Box<dyn BufRead> = match &opt.file
            {
                Some(path) => Box::new(LossyReader::open(path)?),
                None => Box::new(stdin.lock()),
            };
            code_minimap::print(reader, opt.hscale, opt.vscale, opt.padding)?;
        },
    }
    Ok(())
}
