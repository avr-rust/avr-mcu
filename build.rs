use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/// The on-disk path of the crate root.
const CRATE_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

struct Mcu {
    pub name: String,
    pub pack_path: PathBuf,
}

fn pack_dir() -> PathBuf {
    Path::new(CRATE_ROOT).join("packs")
}

fn current_rs_path() -> PathBuf {
    Path::new(CRATE_ROOT).join("src").join("current.rs")
}

fn main() {
    let mcus = self::mcus().expect("failed to find device names");

    let mut current_rs = File::create(current_rs_path()).expect("could not create current.rs");
    self::generate_current_mod(&mcus, &mut current_rs).expect("could not generate current.rs");
}

fn generate_current_mod(mcus: &[Mcu], write: &mut Write) -> Result<(), io::Error> {
    writeln!(write, "//! Constants for the current device")?;
    writeln!(write)?;
    writeln!(write, "// Look up the device by name.")?;
    writeln!(write, "// This is necessary because Rust does not provide a way to stringify")?;
    writeln!(write, "// the value of a cfg flag")?;
    for mcu in mcus {
        writeln!(write, "#[cfg(target_cpu = \"{}\")]", mcu.name)?;
        writeln!(write, "pub const CURRENT_PACK_PATH: &'static str = \"{}\";", mcu.pack_path.display())?;
    }
    writeln!(write)
}

fn mcus() -> Result<Vec<Mcu>, io::Error> {
    let mut mcus = Vec::new();

    // Loop through device collections in $CRATE/packs/<collection>
    for collection_entry in fs::read_dir(pack_dir())? {
        let collection_entry = collection_entry?;

        if collection_entry.path().is_dir() {
            // Loop through actual pack files in $CRATE/packs/<collection>/<device>.atdf
            for entry in fs::read_dir(collection_entry.path())? {
                let entry = entry?;

                if Some("atdf") == entry.path().extension().map(|s| s.to_str().unwrap()) {
                    let mcu_name = entry.path().file_stem().map(|s| s.to_str().unwrap())
                        .unwrap_or("")
                        .to_lowercase()
                        .to_owned();

                    mcus.push(Mcu {
                        name: mcu_name,
                        pack_path: entry.path().to_owned(),
                    });
                }
            }
        }

    }

    Ok(mcus)
}
