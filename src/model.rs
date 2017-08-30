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
    /// A list of supported peripherals.
    pub peripherals: Vec<Peripheral>,
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
pub struct Peripheral {
    /// The name of the peripheral, for example, `PORT`.
    pub name: String,
    /// A list of instances where the peripheral is used.
    pub instances: Vec<Instance>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Module {
    /// The name of the module, for example, `PORT`.
    pub name: String,
    /// Registers associated with the module.
    pub register_groups: Vec<RegisterGroup>,
}

/// An instance of a peripheral.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Instance {
    /// The name of the peripheral instance, for example, `PORTB`.
    pub name: String,
    /// What signals are used in the peripheral.
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

/// A port, such as `PORTB`.
pub struct Port<'a> {
    instance: &'a Instance,
    register_group: &'a RegisterGroup,
}

impl Mcu {
    /// Gets a peripheral module by name.
    pub fn peripheral(&self, name: &str) -> Option<&Peripheral> {
        self.device.peripherals.iter().find(|p| p.name == name)
    }

    /// Gets a module by name.
    pub fn module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|p| p.name == name)
    }

    /// Gets an iterator over all register groups.
    pub fn register_groups<'a>(&'a self) -> impl Iterator<Item=&'a RegisterGroup> {
        self.modules.iter().flat_map(|m| m.register_groups.iter())
    }

    /// Gets an iterator over all registers.
    pub fn registers<'a>(&'a self) -> impl Iterator<Item=&'a Register> {
        self.register_groups().flat_map(|rg| rg.registers.iter())
    }

    /// Gets a port by letter.
    pub fn port(&self, letter: char) -> Port {
        let port_name = format!("PORT{}", letter);
        let instance = self.port_peripheral().instance(&port_name)
            .expect("no port instance with that letter found");
        let register_group = self.port_module().register_group(&port_name)
            .expect("no port register group with that letter found");
        Port { instance, register_group }
    }

    /// Gets the port peripheral.
    pub fn port_peripheral(&self) -> &Peripheral {
        self.peripheral("PORT").expect("mcu does not have a port peripheral")
    }

    /// Gets the port module.
    pub fn port_module(&self) -> &Module {
        self.module("PORT").expect("mcu does not have a port module")
    }
}

impl Peripheral {
    /// Gets an instance by name.
    pub fn instance(&self, name: &str) -> Option<&Instance> {
        self.instances.iter().find(|i| i.name == name)
    }

    /// Gets an iterator over all signals that the peripheral uses.
    pub fn signals<'a>(&'a self) -> impl Iterator<Item=&'a Signal> {
        self.instances.iter().flat_map(|i| i.signals.iter())
    }

    pub fn instance_signal_with_pad(&self, pad: &str)
        -> Option<(&Instance, &Signal)> {
        self.instance_signals_on_pad(pad).next()
    }

    /// Gets a tuple of `(instance, signal)` pairs that use a pad by its name.
    fn instance_signals_on_pad<'a>(&'a self, pad: &str) -> impl Iterator<Item=(&'a Instance, &'a Signal)> {
        let mut instance_signals = Vec::new();

        for instance in self.instances.iter() {
            for signal in instance.signals.iter() {
                if signal.pad == pad {
                    instance_signals.push((instance, signal));
                }
            }
        }
        instance_signals.into_iter()
    }
}

impl Module {
    /// Gets a register group by name.
    pub fn register_group(&self, name: &str) -> Option<&RegisterGroup> {
        self.register_groups.iter().find(|rg| rg.name == name)
    }

    /// Gets an iterator over all registers in the module.
    pub fn registers<'a>(&'a self) -> impl Iterator<Item=&'a Register> {
        self.register_groups.iter().flat_map(|rg| rg.registers.iter())
    }
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

impl<'a> Port<'a> {
    /// Gets all associated registers.
    pub fn registers(&'a self) -> impl Iterator<Item=&'a Register> {
        self.register_group.registers.iter()
    }

    /// Gets all associated signals.
    pub fn signals(&'a self) -> impl Iterator<Item=&'a Signal> {
        self.instance.signals.iter()
    }

    /// Gets the signal associated with a pad.
    pub fn signal_with_pad(&'a self, pad: &str) -> Option<&'a Signal> {
        self.signals().find(|s| s.pad == pad)
    }

    /// Gets the data direction register.
    pub fn ddr_register(&self) -> &Register {
        self.registers().find(|r| r.name.starts_with("DDR")).expect("port does not have ddr register")
    }

    /// Gets the port register.
    pub fn port_register(&self) -> &Register {
        self.registers().find(|r| r.name.starts_with("PORT")).expect("port does not have port register")
    }

    /// Gets the pin register.
    pub fn pin_register(&self) -> &Register {
        self.registers().find(|r| r.name.starts_with("PIN")).expect("port does not have pin register")
    }
}

