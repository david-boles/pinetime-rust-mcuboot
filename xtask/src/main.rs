use anyhow::{anyhow, bail, Context, Error, Result as AResult};
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Message;
use clap::{ArgEnum, Args, IntoApp, Parser, Subcommand};
use object::{Object, ObjectSection};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::Add;
use std::os::unix::prelude::FileExt;
use std::process::{Command, Stdio};
use std::{fs, io};

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
        } => flash(build_opts/*, flash_opts */),
    }
}

fn build(build_opts: Vec<String>) -> AResult<()> {
    build::build(build_opts)?;
    Ok(())
}

fn flash(build_opts: Vec<String>/* , flash_opts: Vec<String>*/) -> AResult<()> {
    let path = build::build(build_opts)?;
    let cargo_executable = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let cargo_command = Command::new(cargo_executable)
        .current_dir(std::env::current_dir()?)
        .arg("flash")
        .args(["--elf", path.as_str()])
        .args(["--chip", "nRF52832_xxAA"])
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

// let bin_end_exc

// let mut bin: Vec<u8> = vec![];
// let mut extend_bin = |name: &str| -> AResult<()> {
//     bin.extend(
//         obj.section_by_name(name)
//             .ok_or(anyhow!(format!("missing section .{}", name)))?
//             .data()?,
//     );
//     Ok(())
// };
// extend_bin(".vector_table")?;
// extend_bin(".text")?;
// extend_bin(".rodata")?;
// May need to include .gnu.sgstubs if linking with GNU tooling?
// TODO check length of all these...

// println!("bin size {}", bin.len());

// Find the header and trailer
// let header_pos = obj.section_by_name(".mcuboot_header");

// let build_bin_path = |suffix| {
//     Result::<Utf8PathBuf, anyhow::Error>::Ok(
//         artifact_path
//             .with_file_name(
//                 artifact_path
//                     .file_stem()
//                     .ok_or(anyhow!("empty artifact file name"))?
//                     .to_string()
//                     + "-"
//                     + suffix,
//             )
//             .with_extension("bin"),
//     )
// };

// // Call the rust toolchain/cargo-binutils provided objcopy to merge the loadable ELF
// // sections into the bin that would be flashed
// let app_bin_path = build_bin_path("app")?;

// // TODO see if we can remove dependency on
// let bin_command = Command::new("rust-objcopy")
//     .current_dir(std::env::current_dir()?)
//     .arg(artifact_path)
//     .args(["-O", "binary"])
//     .arg(&app_bin_path)
//     .stdout(Stdio::piped()) // TODO needed?
//     .spawn()?;
// let output = bin_command.wait_with_output()?;
// if !output.status.success() {
//     bail!(
//         "first rust-objcopy command failed with status code {:?}",
//         output.status.code()
//     )
// }

// let bin = fs::read(app_bin_path)?;

// // Construct the header and trailer
// // See imgtool_notes.md for more details
// let mut header = vec![
//     0x3D, 0xB8, 0xF3, 0x96, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
// ];
// header.extend((bin.len() as u32).to_le_bytes());
// header.extend([0, 0, 0, 0]);
// header.push(0); // Major version number
// header.push(0); // Minor version number
// header.extend((0 as u16).to_le_bytes()); // Patch version number
// header.extend((0 as u32).to_le_bytes()); // Build version number
// header.extend([0, 0, 0, 0]);
// let header_bin_path = build_bin_path("header")?;
// fs::write(&header_bin_path, &header)?;

// let mut footer = vec![0x07, 0x69, 0x28, 0x00, 0x10, 0x00, 0x20, 0x00];
// footer.extend(
//     Sha256::new()
//         .chain_update(&header)
//         .chain_update(&bin)
//         .finalize(),
// );
// let footer_bin_path = build_bin_path("footer")?;
// fs::write(footer_bin_path, &footer)?;

// // Add header and footer to the cargo-generated ELF
// let elf_command = Command::new("rust-objcopy")
//     .current_dir(std::env::current_dir()?)
//     .arg(artifact_path)
//     .args([
//         "--add-section",
//         format!(".header={}", &header_bin_path).as_str(),
//         "--set-section-flags=.header=load",
//     ])
//     .stdout(Stdio::piped()) // TODO needed?
//     .spawn()?;
// let output = elf_command.wait_with_output()?;
// if !output.status.success() {
//     bail!(
//         "second rust-objcopy command failed with status code {:?}",
//         output.status.code()
//     )
// }

// -----START REALLY OLD ----

// hasher.update(&header);
// hasher.update(&bin);
// let hash = hasher.

// header.extend
//      : IMAGE MAGIC                                                                 =8s.........@...
// 00000000:             00 00 00 00 : LOAD ADDRESS                                                    =8s.........@...
// 00000000:                         20 00 : HEADER SIZE (32)                                          =8s.........@...
// 00000000:                               00 00

// println!("{}\n{:?}", &bin_path, output.stdout);

// println!("{}\n{:?}", bin_path, output.stdout);

// TODO use cargo-binutils' rust-objcopy
// rust-objcopy target/thumbv7em-none-eabihf/debug/blinky-button-demo -O binary target/thumbv7em-none-eabihf/debug/blinky-button-demo-raw.bin
// To get rust binary
// Read that, create header, hash, create trailer
// Overwrite elf
// rust-objcopy target/thumbv7em-none-eabihf/debug/blinky-button-demo --add-section ...

// let bin_data = fs::read(target_artifact.executable.unwrap().as_path())?; // safe, executable is known to be Some
// let obj_file = object::File::parse(&*bin_data)?;

// if let Some(section) = obj_file.section_by_name(".boot") {
//     println!("{:#x?}", section.data()?);
// } else {
//     eprintln!("section not available");
// }
// println!("{:?}", obj_file.entry());
