use model::Mcu;
use pack;

use std::path::{Path, PathBuf};
use std::{fs, io};

/// The extension on the pack files.
const PACK_FILE_EXT: &str = "atdf";

/// All pack collections inside the 'packs' folder
/// of this repository.
const PACK_COLLECTIONS: &[&str] =
    &["atmega", "tiny", "xmegaa", "xmegab", "xmegac", "xmegad", "xmegae", "automotive"];

/// The on-disk path of the crate root.
const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

lazy_static! {
    static ref MCUS: Vec<Mcu> =
        self::load_microcontrollers().expect("failed to load microcontrollers");
    static ref MCU_NAMES: Vec<String> = pack_informations()
        .expect("could not find packfiles")
        .into_iter()
        .map(|pack| pack.mcu_name)
        .collect();
}

struct PackInfo {
    pub mcu_name: String,
    pub path: PathBuf,
}

/// Retrieves a list of `Mcu` objects for all microcontrollers.
pub fn microcontrollers() -> &'static [Mcu] {
    &MCUS[..]
}

/// Retrieves a list of all microcontroller names.
///
/// # Examples
///
/// * `atmega328p`
/// * `attiny85`
pub fn microcontroller_names() -> &'static [String] {
    &MCU_NAMES[..]
}

/// Retrieves information for a specific microcontroller.
pub fn microcontroller(name: &str) -> Mcu {
    let pack_info = pack_informations()
        .unwrap()
        .into_iter()
        .find(|pack_info| pack_info.mcu_name == name)
        .expect(&format!("no microcontroller with the name '{}' found", name));
    pack::load(&pack_info.path).expect("could not parse microcontroller pack")
}

/// Retrieves a list of `Mcu` objects in a directory containg `PACK_COLLECTIONS`.
fn load_microcontrollers() -> Result<Vec<Mcu>, io::Error> {
    let microcontrollers = pack_informations()?
        .into_iter()
        .map(|pack_info| pack::load(&pack_info.path).unwrap())
        .collect();

    Ok(microcontrollers)
}

fn pack_informations() -> Result<Vec<PackInfo>, io::Error> {
    let path = Path::new(CRATE_ROOT).join("packs");
    pack_informations_from(&path)
}

fn pack_informations_from(path: &Path) -> Result<Vec<PackInfo>, io::Error> {
    let mut pack_paths = Vec::new();

    for pack_name in PACK_COLLECTIONS {
        pack_paths.extend(find_packs(&path.join(pack_name)).unwrap());
    }

    Ok(pack_paths
        .into_iter()
        .map(|path| PackInfo {
            mcu_name: path.file_stem().unwrap().to_str().unwrap().to_lowercase().to_owned(),
            path: path.to_owned(),
        })
        .collect())
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

    #[test]
    fn can_get_atmega328p_by_name() {
        let mcu = super::microcontroller("atmega328p");
        assert_eq!("ATmega328P", mcu.device.name);
    }
}
