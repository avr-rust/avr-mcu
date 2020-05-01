/// A microcontroller with one or more variants.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Mcu {
    /// Information about the microcontroller itself.
    pub device: Device,
    /// The different variants the mcu can come in.
    pub variants: Vec<Variant>,
    /// The modules built into the mcu package.
    pub modules: Vec<Module>,
    /// The family that the mcu belongs to.
    pub architecture: Architecture,
    /// The C preprocessor name.
    pub c_preprocessor_name: String,
}

/// Information fore a specific device.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Device {
    /// The name of the device.
    pub name: String,
    /// A list of all address spaces the device has.
    pub address_spaces: Vec<AddressSpace>,
    /// A list of supported peripherals.
    pub peripherals: Vec<Peripheral>,
    /// A list of supported interrupts
    pub interrupts: Vec<Interrupt>,
}

/// A variation of a specific microcontroller.
#[derive(Clone, Debug, PartialOrd, PartialEq)]
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
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
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
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct MemorySegment {
    /// The name of the segment.
    pub name: String,
    /// A pointer to the first byte in the segment.
    pub start_address: u32,
    /// The number of bytes in the segment.
    pub size: u32,
    /// The segment type.
    pub ty: String,
    /// Whether the segment can be read from.
    pub readable: bool,
    /// Whether the segment can be written to.
    pub writable: bool,
    /// Whether the segment can be executed.
    pub executable: bool,
    /// How large pages are in this segment.
    pub page_size: Option<u32>,
}

/// An on-board peripheral, such as an IO port.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Peripheral {
    /// The name of the peripheral, for example, `PORT`.
    pub name: String,
    /// A list of instances where the peripheral is used.
    ///
    /// As an example, if the peripheral is an IO port, then the
    /// instance list would list all PORT instances, such as `PORTA`
    /// and `PORTB`.
    pub instances: Vec<Instance>,
}

/// An interrupt supported by a device.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Interrupt {
    /// The name of the interrupt, for example `TIMER1_COMPA`.
    pub name: String,
    /// A brief description of the interrupt
    pub caption: String,
    /// The interrupt vector table index
    pub index: u32,
}

/// A module built into the silicon.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Module {
    /// The name of the module, for example, `PORT`.
    pub name: String,
    /// Registers associated with the module.
    pub register_groups: Vec<RegisterGroup>,
    /// Value groups associated with the module.
    pub value_groups: Vec<ValueGroup>,
}

/// An instance of a peripheral.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Instance {
    /// The name of the peripheral instance, for example, `PORTB`.
    pub name: String,
    /// What signals are used in the peripheral.
    pub signals: Vec<Signal>,
}

/// A group of registers.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct RegisterGroup {
    /// The name of the group.
    pub name: String,
    pub caption: String,
    /// The registers that make up the group.
    pub registers: Vec<Register>,
}

/// A group of values.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct ValueGroup {
    pub name: String,
    pub caption: String,
    pub values: Vec<Value>,
}

/// A values for a register/mask.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Value {
    pub name: String,
    pub caption: String,
    pub value: u32,
}

/// Specifies the mutability of a register.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub enum ReadWrite {
    /// The register is readable and writable.
    ReadAndWrite,
    /// The register is read-only.
    ReadOnly,
    /// The register is write-only.
    WriteOnly,
}

/// An CPU or IO register.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Register {
    /// The name of the register, such as `TCCR0A`.
    pub name: String,
    /// The register description.
    pub caption: String,
    /// The offset of the register in IO space.
    pub offset: u32,
    /// The number of bytes that make up the bitfield.
    pub size: u32,
    pub mask: Option<u32>,
    /// The mutability of the register.
    pub rw: ReadWrite,
    /// The bitfields supported by the register.
    pub bitfields: Vec<Bitfield>,
}

/// A bitfield within a register.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Bitfield {
    /// The name of the bitfield, such as `U2X0` for the USART register `UCSR0A`.
    pub name: String,
    /// A description of the bitfield.
    pub caption: String,
    /// The mask that makes up the bitfield.
    ///
    /// This will always match up to the parent register. A 16-bit register has
    /// 16-but masks.
    pub mask: u32,
    /// The number of bytes that make up the bitfield.
    pub size: u32,
    /// reference into value_groups on the container
    pub values: Option<String>,
}

/// A signal that is exposed on the outside of the package.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Signal {
    /// The external pin name that exposes the signal.
    pub pad: String,
    pub group: Option<String>,
    pub index: Option<u8>,
}

/// An AVR architecture (mcu family) name.
///
/// Architecture is a misnomer - 'mcu family' would make sense.
/// Cores with the same instruction sets share an architecture name.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Architecture {
    Unknown,

    Avr0,
    Avr1,
    Avr2,
    Avr25,
    Avr3,
    Avr31,
    Avr35,
    Avr4,
    Avr5,
    Avr51,
    Avr6,
    Xmega2,
    Xmega3,
    Xmega4,
    Xmega5,
    Xmega6,
    Xmega7,
    Tiny,
}

/// A port, such as `PORTB`.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub struct Port<'a> {
    /// The port peripheral instance.
    instance: &'a Instance,
    /// The register group associated with the port.
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
    pub fn register_groups<'a>(&'a self) -> impl Iterator<Item = &'a RegisterGroup> {
        self.modules.iter().flat_map(|m| m.register_groups.iter())
    }

    /// Gets an iterator over all registers.
    pub fn registers<'a>(&'a self) -> impl Iterator<Item = &'a Register> {
        self.register_groups().flat_map(|rg| rg.registers.iter())
    }

    /// Gets a port by letter.
    pub fn port(&self, letter: char) -> Port {
        let port_name = format!("PORT{}", letter);
        let instance = self
            .port_peripheral()
            .instance(&port_name)
            .expect("no port instance with that letter found");
        let register_group = self
            .port_module()
            .register_group(&port_name)
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
    pub fn signals<'a>(&'a self) -> impl Iterator<Item = &'a Signal> {
        self.instances.iter().flat_map(|i| i.signals.iter())
    }

    pub fn instance_signal_with_pad(&self, pad: &str) -> Option<(&Instance, &Signal)> {
        self.instance_signals_on_pad(pad).next()
    }

    /// Gets a tuple of `(instance, signal)` pairs that use a pad by its name.
    fn instance_signals_on_pad<'a>(
        &'a self,
        pad: &str,
    ) -> impl Iterator<Item = (&'a Instance, &'a Signal)> {
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
    pub fn registers<'a>(&'a self) -> impl Iterator<Item = &'a Register> {
        self.register_groups.iter().flat_map(|rg| rg.registers.iter())
    }
}

impl Register {
    /// Get the union between two descriptions of the same register.
    pub fn union(&self, with: &Self) -> Self {
        assert_eq!(
            self.name, with.name,
            "can only take the union between different descriptions of the same register"
        );

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
    pub fn registers(&'a self) -> impl Iterator<Item = &'a Register> {
        self.register_group.registers.iter()
    }

    /// Gets all associated signals.
    pub fn signals(&'a self) -> impl Iterator<Item = &'a Signal> {
        self.instance.signals.iter()
    }

    /// Gets the signal associated with a pad.
    pub fn signal_with_pad(&'a self, pad: &str) -> Option<&'a Signal> {
        self.signals().find(|s| s.pad == pad)
    }

    /// Gets the data direction register.
    pub fn ddr_register(&self) -> &Register {
        self.registers()
            .find(|r| r.name.starts_with("DDR"))
            .expect("port does not have ddr register")
    }

    /// Gets the port register.
    pub fn port_register(&self) -> &Register {
        self.registers()
            .find(|r| r.name.starts_with("PORT"))
            .expect("port does not have port register")
    }

    /// Gets the pin register.
    pub fn pin_register(&self) -> &Register {
        self.registers()
            .find(|r| r.name.starts_with("PIN"))
            .expect("port does not have pin register")
    }
}

impl Architecture {
    pub fn name(&self) -> &'static str {
        use Architecture::*;

        match self {
            Unknown => "<unknown architecture>",
            Avr0 => "avr0",
            Avr1 => "avr1",
            Avr2 => "avr2",
            Avr25 => "avr25",
            Avr3 => "avr3",
            Avr31 => "avr31",
            Avr35 => "avr35",
            Avr4 => "avr4",
            Avr5 => "avr5",
            Avr51 => "avr51",
            Avr6 => "avr6",
            Xmega2 => "xmega2",
            Xmega3 => "xmega3",
            Xmega4 => "xmega4",
            Xmega5 => "xmega5",
            Xmega6 => "xmega6",
            Xmega7 => "xmega7",
            Tiny => "tiny",
        }
    }
}
