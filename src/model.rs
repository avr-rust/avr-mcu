/// A microcontroller with one or more variants.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Mcu {
    /// Information about the microcontroller itself.
    pub device: Device,
    /// The different variants the mcu can come in.
    pub variants: Vec<Variant>,
    /// The modules built into the mcu package.
    pub modules: Vec<Module>,
}

/// Information fore a specific device.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Device {
    /// The name of the device.
    pub name: String,
    /// A list of all address spaces the device has.
    pub address_spaces: Vec<AddressSpace>,
    /// A list of supported modules.
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
/// A variation of a specific microcontroller.
pub struct Variant {
    /// The name of the variant.
    pub name: String,
    /// What pinout is used.
    pub pinout: Option<String>,
    /// The package format.
    pub package: String,
    /// The minimum temperate in celsius.
    pub temperature_min: i32,
    /// The maximum temperature in celsius.
    pub temperature_max: i32,
    /// The minimum voltage.
    pub voltage_min: f32,
    /// The maximum voltate.
    pub voltage_max: f32,
    /// The max clock speed in Hz.
    pub speed_max_hz: u64,
}

/// An address space.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct AddressSpace {
    /// The identifier.
    pub id: String,
    /// The name.
    pub name: String,
    /// The starting memory address of the address space.
    pub start_address: u32,
    /// The number of bytes in the address space.
    pub size: u32,
    /// What segments are in the address space.
    pub segments: Vec<MemorySegment>,
}

/// A segment of memory in a particular address space.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct MemorySegment {
    pub start_address: u32,
    pub size: u32,
    pub ty: String,
    /// Whether the segment can be read from.
    pub readable: bool,
    /// Whether the segment can be written to.
    pub writable: bool,
    /// Whether the segment can be executed.
    pub executable: bool,
    pub name: String,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Module {
    /// The name of the module, for example, `PORT`.
    pub name: String,
    /// A list of instances where the module is used.
    pub instances: Vec<Instance>,
    /// Registers associated with the module.
    pub register_groups: Vec<RegisterGroup>,
}

/// An instance of a module.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Instance {
    /// The name of the module instance, for example, `PORTB`.
    pub name: String,
    /// What signals are used in the module.
    pub signals: Vec<Signal>,
}

/// A group of registers.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct RegisterGroup {
    pub name: String,
    pub caption: String,
    pub registers: Vec<Register>,
}

/// An CPU or IO register.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Register {
    pub name: String,
    pub caption: String,
    pub offset: u32,
    pub size: u32,
    pub mask: Option<u32>,
}

/// A signal that is exposed on the outside of the package.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
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

