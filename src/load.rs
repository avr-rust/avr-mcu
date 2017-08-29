use model::Mcu;
use pack;

use std::path::{Path, PathBuf};
use std::{fs, io};

/// The extension on the pack files.
const PACK_FILE_EXT: &'static str = "atdf";

/// All pack collections inside the 'packs' folder
/// of this repository.
const PACK_COLLECTIONS: &'static [&'static str] = &[
    "atmega", "tiny", "xmegaa", "xmegab",
    "xmegac", "xmegad", "xmegae", "automotive",
];

/// The on-disk path of the crate root.
const CRATE_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

lazy_static! {
    static ref MCUS: Vec<Mcu> = self::load_microcontrollers();
}

/// Retrieves a list of `Mcu` objects for all microcontrollers.
pub fn microcontrollers() -> &'static [Mcu] {
    &MCUS[..]
}

/// Loads all microcontrollers.
pub fn load_microcontrollers() -> Vec<Mcu> {
    let path = Path::new(CRATE_ROOT).join("packs");
    load_microcontrollers_from(&path)
        .expect("error whilst reading microcontrollers")
}

/// Retrieves a list of `Mcu` objects in a directory containg `PACK_COLLECTIONS`.
fn load_microcontrollers_from(path: &Path) -> Result<Vec<Mcu>, io::Error> {
    let mut pack_paths = Vec::new();

    for pack_name in PACK_COLLECTIONS {
        pack_paths.extend(find_packs(&path.join(pack_name)).unwrap());
    }

    Ok(pack_paths.into_iter().map(|path| pack::load(&path).unwrap()).collect())
}

/// Finds all pack files in a directory.
fn find_packs(in_dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(in_dir)? {
        let entry = entry?;
        if let Some(PACK_FILE_EXT) = entry.path().extension().map(|s| s.to_str().unwrap()) {
            paths.push(entry.path());
        }
    }
    Ok(paths)
}

#[cfg(test)]
mod test {
    #[test]
    fn there_are_at_least_100_microcontrollers() {
        let mcus = super::microcontrollers();
        assert!(mcus.len() > 100, "there should be at least 100 microcontrollers");
    }
}

