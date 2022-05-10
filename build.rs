use clap::{ArgEnum, IntoApp};
use std::{fs, path::Path};

include!("src/bin/code-minimap/cli.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let outdir = std::env::var_os("SHELL_COMPLETIONS_DIR")
        .or_else(|| std::env::var_os("OUT_DIR"))
        .expect("OUT_DIR not found");
    let outdir_path = Path::new(&outdir);
    let cmd = &mut App::command();

    for shell in Shell::value_variants() {
        let dir = outdir_path.join(shell.to_string());
        fs::create_dir_all(&dir)?;
        clap_complete::generate_to(*shell, cmd, cmd.get_name().to_string(), &dir)?;
    }
    Ok(())
}
