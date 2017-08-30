use super::*;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path ;

use xmltree::Element;

pub fn load(path: &Path) -> Result<Mcu, io::Error> {
    let mut file = File::open(path)?;
    let mut body = String::new();
    file.read_to_string(&mut body)?;

    println!("loading pack '{}'", path.display());
    let root = Element::parse(body.as_bytes()).unwrap();

    Ok(self::read_pack(&root))
}

fn read_pack(root: &Element) -> Mcu {
    let device_element = root.get_child("devices").unwrap().get_child("device").unwrap();

    let device = self::read_device(&device_element);
    let variants = root.get_child("variants").unwrap().children.iter().map(self::read_variant).collect();
    let modules = root.get_child("modules").unwrap().children.iter().map(self::read_module);

    Mcu {
        device: device,
        variants: variants,
        modules: modules.collect(),
    }
}

fn read_device(device: &Element) -> Device {
    let device_name = device.attributes.get("name").unwrap().clone();

    let peripherals = device.get_child("peripherals").unwrap()
                        .children.iter()
                        .map(self::read_peripheral)
                        .collect();

    let address_spaces = device.get_child("address-spaces")
                               .unwrap().children.iter()
                               .map(self::read_address_space)
                               .collect();

    Device {
        name: device_name,
        address_spaces: address_spaces,
        peripherals,
    }
}

fn read_peripheral(module: &Element) -> Peripheral {
    let name = module.attributes.get("name").unwrap().clone();
    let mut instances = Vec::new();

    for child in module.children.iter() {
        match &child.name[..] {
            "instance" => instances.push(read_instance(child)),
            // Unimplemented tags.
            _ => (),
        }
    }

    Peripheral { name, instances }
}

fn read_module(module: &Element) -> Module {
    let module_name = module.attributes.get("name").unwrap().clone();
    let mut register_groups = Vec::new();

    for child in module.children.iter() {
        match &child.name[..] {
            "register-group" => register_groups.push(read_register_group(child)),
            // Unimplemented tags.
            _ => (),
        }
    }

    Module {
        name: module_name,
        register_groups: register_groups,
    }
}

fn read_variant(variant: &Element) -> Variant {
    Variant {
        name: variant.attributes.get("ordercode").unwrap().clone(),
        temperature_min: variant.attributes.get("tempmin").unwrap().parse().unwrap(),
        temperature_max: variant.attributes.get("tempmax").unwrap().parse().unwrap(),
        voltage_min: variant.attributes.get("vccmin").unwrap().parse().unwrap(),
        voltage_max: variant.attributes.get("vccmax").unwrap().parse().unwrap(),
        package: variant.attributes.get("package").unwrap().clone(),
        pinout: variant.attributes.get("pinout").map(|p| p.clone()),
        speed_max_hz: variant.attributes.get("speedmax").unwrap().parse().unwrap(),
    }
}

fn read_instance(instance: &Element) -> Instance {
    let instance_name = instance.attributes.get("name").unwrap().clone();

    let signals = match instance.get_child("signals") {
        Some(signals) => signals.children.iter().map(read_signal).collect(),
        None =>  Vec::new(),
    };

    Instance {
        name: instance_name,
        signals: signals,
    }
}

fn read_signal(signal: &Element) -> Signal {
    Signal {
        pad: signal.attributes.get("pad").unwrap().clone(),
        group: signal.attributes.get("group").map(|p| p.clone()),
        index: signal.attributes.get("index").map(|i| i.parse().unwrap()),
    }
}

/// Reads a register group.
///
/// This looks like so
///
/// ```xml
/// <register-group caption="EEPROM" name="EEPROM">
///   <register caption="EEPROM Address Register  Bytes" name="EEAR" offset="0x41" size="2" mask="0x01FF"/>
///   <register caption="EEPROM Data Register" name="EEDR" offset="0x40" size="1" mask="0xFF"/>
/// </register-group>
fn read_register_group(register_group: &Element) -> RegisterGroup {
    let (name, caption) = (register_group.attributes.get("name").unwrap(),
                           register_group.attributes.get("caption").unwrap());
    let registers = register_group.children.iter().filter_map(|child| match &child.name[..] {
        "register" => Some(self::read_register(child)),
        // FIXME: leave this out for now, ATtiny816 has nested register-group
        // _ => panic!("unknown register-group child: '{}'", child.name),
        _ => None,
    }).collect();

    RegisterGroup {
        name: name.clone(),
        caption: caption.clone(),
        registers: registers,
    }
}

/// Reads a register.
///
/// This looks like
///
/// ```xml
/// <register caption="EEPROM Address Register  Bytes" name="EEAR" offset="0x41" size="2" mask="0x01FF"/>
/// ```
fn read_register(register: &Element) -> Register {
    Register {
        name: register.attributes.get("name").unwrap().clone(),
        caption: register.attributes.get("caption").unwrap().clone(),
        offset: read_int(register.attributes.get("offset")).clone(),
        size: register.attributes.get("size").unwrap().parse().unwrap(),
        mask: read_opt_int(register.attributes.get("mask")).clone()
    }
}

/// Reads an eddress space.
///
/// This looks like
///
/// ```xml
/// <address-space endianness="little" name="signatures" id="signatures" start="0" size="3">
///   <memory-segment start="0" size="3" type="signatures" rw="R" exec="0" name="SIGNATURES"/>
/// </address-space>
/// ```
fn read_address_space(address_space: &Element) -> AddressSpace {
    let id = address_space.attributes.get("id").unwrap().clone();
    let start_address = read_int(address_space.attributes.get("start"));
    let size = read_int(address_space.attributes.get("size"));
    let segments = address_space.children.iter().map(read_memory_segment).collect();

    AddressSpace {
        id: id,
        name: address_space.attributes.get("name").unwrap().clone(),
        start_address: start_address,
        size: size,
        segments: segments,
    }
}

/// Reads a memory segment.
///
/// ```xml
/// <memory-segment start="0" size="3" type="signatures" rw="R" exec="0" name="SIGNATURES"/>
/// ```
fn read_memory_segment(memory_segment: &Element) -> MemorySegment {
    let default_perms = "".to_owned();

    let start_address = read_int(memory_segment.attributes.get("start"));
    let size = read_int(memory_segment.attributes.get("size"));
    let ty = memory_segment.attributes.get("type").unwrap().clone();
    let rw = memory_segment.attributes.get("rw").unwrap_or(&default_perms);
    let exec = memory_segment.attributes.get("exec").unwrap_or(&default_perms);
    let name = memory_segment.attributes.get("name").unwrap().clone();
    let page_size = memory_segment.attributes.get("pagesize").map(|p| read_int(Some(p)));

    let readable = rw.contains("r") || rw.contains("R");
    let writable = rw.contains("w") || rw.contains("W");
    let executable = exec == "1";

    MemorySegment {
        start_address, size, ty, name, readable, writable, executable,
        page_size
    }
}

fn read_int(value: Option<&String>) -> u32 {
    let value = value.unwrap();

    if value.starts_with("0x") {
        read_hex(Some(value))
    } else {
        value.parse().unwrap()
    }
}

fn read_opt_int(value: Option<&String>) -> Option<u32> {
    value.map(|v| {
        if v.starts_with("0x") {
            read_hex(Some(v))
        } else {
            v.parse().unwrap()
        }
    })
}

fn read_hex(value: Option<&String>) -> u32 {
    let value = value.unwrap().replacen("0x", "", 1);
    u32::from_str_radix(&value, 16).unwrap()
}

