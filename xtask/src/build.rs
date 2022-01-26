use anyhow::{anyhow, bail, Context, Error, Result as AResult};
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Message;
use clap::{ArgEnum, Args, Parser, Subcommand};
use object::{Object, ObjectSection, ObjectSegment, SegmentFlags};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::{Add, Range};
use std::os::unix::prelude::FileExt;
use std::process::{Command, Stdio};
use std::{
    cmp::{max, min},
    fs, io,
};

/// Build the project and then augment the mcuboot sections with their data.
pub fn build(build_opts: Vec<String>) -> AResult<Utf8PathBuf> {
    // Based on https://github.com/probe-rs/probe-rs which is dual Apache/MIT licensed
    let mut firmware_dir = std::env::current_dir()?;
    firmware_dir.push("firmware");
    let cargo_executable = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let cargo_command = Command::new(cargo_executable)
        .current_dir(firmware_dir)
        .arg("build")
        .args(["--message-format", "json-diagnostic-rendered-ansi"])
        .args(["--target", "thumbv7em-none-eabihf"])
        .args(build_opts)
        .stdout(Stdio::piped())
        .spawn()?;
    let output = cargo_command.wait_with_output()?;

    if !output.status.success() {
        bail!(
            "cargo build command failed with status code {:?}",
            output.status.code()
        )
    }

    // Parse the cargo build command output to find the generated artifact
    let mut path = None;

    let messages = Message::parse_stream(&output.stdout[..]);
    for message in messages {
        match message? {
            Message::CompilerArtifact(artifact) => {
                // We only want artifacts with an executable, and we only expect one to be built
                match artifact.executable {
                    Some(executable) => {
                        if path.is_some() {
                            bail!("multiple artifacts were found");
                        } else {
                            path = Some(executable);
                        }
                    }
                    None => {}
                }
            }
            Message::CompilerMessage(message) => {
                if let Some(rendered) = message.message.rendered {
                    print!("{}", rendered);
                }
            }
            // Ignore other messages.
            _ => (),
        }
    }

    let path = match path {
        None => bail!("no artifact was built"),
        Some(path) => path,
    };

    println!("{}", &path);

    // Open the the built artifact, then load and parse its current contents
    let mut file = OpenOptions::new().read(true).write(true).open(&path)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    let obj = object::File::parse(&*contents)?;

    let header = obj
        .section_by_name(".mcuboot_header")
        .ok_or(anyhow!("missing section .mcuboot_header"))?;
    let trailer = obj
        .section_by_name(".mcuboot_trailer")
        .ok_or(anyhow!("missing section .mcuboot_trailer"))?;

    // Construct the raw program bin
    let bin_addr = header.address() + header.size();
    let bin_limit = trailer.address();
    let mut bin = vec![0; (bin_limit - bin_addr) as usize];
    for segment in obj.segments() {
        // All of these segments will be PT_LOAD
        let seg_addr = segment.address();
        let seg_size = segment.size();
        let seg_limit = seg_addr + seg_size;

        // Only copy over the parts of segments that lie in the bin region.
        if seg_addr >= bin_limit || seg_limit <= bin_addr {
            continue;
        }
        let included_addr = max(seg_addr, bin_addr);
        let included_limit = min(seg_addr + seg_size, bin_limit);

        let bin_slice = &mut bin[offset_range(bin_addr, included_addr, included_limit)];
        let included_slice =
            &segment.data()?[offset_range(seg_addr, included_addr, included_limit)];

        bin_slice.copy_from_slice(included_slice);
    }

    // Note the header and trailer data locations in the file
    let header_range = header
        .file_range()
        .ok_or(anyhow!("header does not have a file range"))?;
    let trailer_range = trailer
        .file_range()
        .ok_or(anyhow!("trailer does not have a file range"))?;

    // Generate and fill header and trailer section data
    // See imgtool_notes.md for more details
    let mut header = vec![
        0x3D, 0xB8, 0xF3, 0x96, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
    ];
    header.extend((bin.len() as u32).to_le_bytes());
    header.extend([0, 0, 0, 0]);
    // TODO make these configurable
    header.push(0); // Major version number
    header.push(0); // Minor version number
    header.extend((0 as u16).to_le_bytes()); // Patch version number
    header.extend((0 as u32).to_le_bytes()); // Build version number
    header.extend([0, 0, 0, 0]);
    assert_eq!(header_range.1, header.len() as u64);
    file.write_all_at(&header, header_range.0)?;

    let mut trailer = vec![0x07, 0x69, 0x28, 0x00, 0x10, 0x00, 0x20, 0x00];
    trailer.extend(
        Sha256::new()
            .chain_update(&header)
            .chain_update(&bin)
            .finalize(),
    );
    assert_eq!(trailer_range.1, trailer.len() as u64);
    file.write_all_at(&trailer, trailer_range.0)?;

    // Finish writing the binary to disk and return with its path.
    file.sync_all()?;

    // // TODO temp
    // let mut file = OpenOptions::new().write(true).open("./temp")?;
    // file.write_all(&header)?;
    // file.write_all(&bin)?;
    // file.write_all(&trailer)?;

    Ok(path)
}

fn offset_range(offset: u64, address: u64, limit: u64) -> Range<usize> {
    (address - offset) as usize..(limit - offset) as usize
}
