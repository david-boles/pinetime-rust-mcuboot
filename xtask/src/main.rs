use anyhow::{bail, Result as AResult};
use clap::{IntoApp, Parser, Subcommand};
use std::process::{Command, Stdio};

mod build;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
// #[clap(APP ATTRIBUTE)]
struct Cli {
    #[clap(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Run `cargo build` and then augment the binary with an mcuboot header and trailer.
    Build {
        /// Any additional aguments you'd like to pass to `cargo build`.
        build_opts: Vec<String>,
    },
    Flash {
        /// Any additional aguments you'd like to pass to `cargo build`.
        build_opts: Vec<String>,
        // TODO:
        // / Any additional aguments you'd like to pass to `cargo flash`.
        // flash_opts: Vec<String>,
    },
}

fn main() -> AResult<()> {
    Cli::into_app().debug_assert();
    let cli = Cli::parse();
    match cli.command {
        CliCommand::Build { build_opts } => build(build_opts),
        CliCommand::Flash {
            build_opts,
            // flash_opts,
        } => flash(build_opts /*, flash_opts */),
    }
}

fn build(build_opts: Vec<String>) -> AResult<()> {
    build::build(build_opts)?;
    Ok(())
}

fn flash(build_opts: Vec<String> /* , flash_opts: Vec<String>*/) -> AResult<()> {
    let path = build::build(build_opts)?;
    let cargo_executable = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let cargo_command = Command::new(cargo_executable)
        .current_dir(std::env::current_dir()?)
        .arg("flash")
        .args(["--elf", path.as_str()])
        .args(["--chip", "nRF52832_xxAA"])
        .arg("--restore-unwritten")
        // .args(flash_opts)
        .stdout(Stdio::piped())
        .spawn()?;
    let output = cargo_command.wait_with_output()?;

    if !output.status.success() {
        bail!(
            "cargo flash command failed with status code {:?}",
            output.status.code()
        )
    }

    Ok(())
}
