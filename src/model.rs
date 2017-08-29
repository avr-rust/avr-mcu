#[derive(Clone, Debug)]
pub struct Pack {
    pub device: Device,
    pub variants: Vec<Variant>,
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    pub address_spaces: Vec<AddressSpace>,
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: String,
    pub pinout: Option<String>,
    pub package: String,
    pub temperature_min: i32,
    pub temperature_max: i32,
    pub voltage_min: f32,
    pub voltage_max: f32,
    pub speed_max_hz: u64,
}

#[derive(Clone, Debug)]
pub struct AddressSpace {
    pub id: String,
    pub name: String,
    pub start_address: u32,
    pub size: u32,
    pub segments: Vec<MemorySegment>,
}

#[derive(Clone, Debug)]
pub struct MemorySegment {
    pub start_address: u32,
    pub size: u32,
    pub ty: String,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub name: String,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub instances: Vec<Instance>,
    pub register_groups: Vec<RegisterGroup>,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub name: String,
    pub signals: Vec<Signal>,
}

#[derive(Clone, Debug)]
pub struct RegisterGroup {
    pub name: String,
    pub caption: String,
    pub registers: Vec<Register>,
}

#[derive(Clone, Debug)]
pub struct Register {
    pub name: String,
    pub caption: String,
    pub offset: u32,
    pub size: u32,
    pub mask: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Signal {
    pub pad: String,
    pub group: Option<String>,
    pub index: Option<u8>,
}

impl Register {
    /// Get the union between two descriptions of the same register.
    pub fn union(&self, with: &Self) -> Self {
        assert_eq!(self.name, with.name,
                   "can only take the union between different descriptions of the same register");

        let mut result = self.clone();

        match (result.mask, with.mask) {
            (None, Some(v)) => result.mask = Some(v), // rhs is more specific
            _ => (),
        }

        result
    }
}

