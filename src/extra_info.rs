//! Extra MCU information that does not exist in packfiles.

use Architecture;

/// Information about a MCU.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Info {
    /// The architecture.
    pub arch: Architecture,
    pub c_preprocessor_name: String,
}

fn mmcu_from_mcu_name(mcu_name: &str) -> Architecture {
    use Architecture::*;

    match mcu_name {
        "AT90S1200" | "ATtiny11" | "ATtiny12" | "ATtiny15" | "ATtiny28" => Avr1,

        "AT90S2313" | "AT90S2323" | "AT90S2333" | "AT90S2343" | "ATtiny22" | "ATtiny26"
        | "AT90S4414" | "AT90S4433" | "AT90S4434" | "AT90S8515" | "AT90S8534" | "AT90S8535" => Avr2,

        "ATA5272" | "ATA6616C" | "ATtiny13" | "ATtiny13A" | "ATtiny2313" | "ATtiny2313A"
        | "ATtiny24" | "ATtiny24A" | "ATtiny4313" | "ATtiny44" | "ATtiny44A" | "ATtiny441"
        | "ATtiny84" | "ATtiny84A" | "ATtiny25" | "ATtiny45" | "ATtiny85" | "ATtiny261"
        | "ATtiny261A" | "ATtiny461" | "ATtiny461A" | "ATtiny861" | "ATtiny861A" | "ATtiny43U"
        | "ATtiny87" | "ATtiny48" | "ATtiny88" | "ATtiny828" | "ATtiny841" | "AT86RF401" => Avr25,

        "AT43USB355" | "AT76C711" => Avr3,

        "ATmega103" | "AT43USB320" => Avr31,

        "ATA5505" | "ATA6617C" | "ATA664251" | "AT90USB82" | "AT90USB162" | "ATmega8U2"
        | "ATmega16U2" | "ATmega32U2" | "ATtiny167" | "ATtiny1634" => Avr35,

        "ATA6285" | "ATA6286" | "ATA6289" | "ATA6612C" | "ATmega8" | "ATmega8A" | "ATmega48"
        | "ATmega48A" | "ATmega48P" | "ATmega48PA" | "ATmega48PB" | "ATmega88" | "ATmega88A"
        | "ATmega88P" | "ATmega88PA" | "ATmega88PB" | "ATmega8515" | "ATmega8535"
        | "ATmega8HVA" | "AT90PWM1" | "AT90PWM2" | "AT90PWM2B" | "AT90PWM3" | "AT90PWM3B"
        | "AT90PWM81" => Avr4,

        "ATA5700M322" | "ATA5702M322" | "ATA5782" | "ATA5790" | "ATA5790N" | "ATA5791"
        | "ATA5795" | "ATA5831" | "ATA6613C" | "ATA6614Q" | "ATA8210" | "ATA8215" | "ATA8510"
        | "ATmega16" | "ATmega16A" | "ATmega161" | "ATmega162" | "ATmega163" | "ATmega164A"
        | "ATmega164P" | "ATmega164PA" | "ATmega165" | "ATmega165A" | "ATmega165P"
        | "ATmega165PA" | "ATmega168" | "ATmega168A" | "ATmega168P" | "ATmega168PA"
        | "ATmega168PB" | "ATmega169" | "ATmega169A" | "ATmega169P" | "ATmega169PA"
        | "ATmega16HVB" | "ATmega16HVBREVB" | "ATmega16M1" | "ATmega16U4" | "ATmega32A"
        | "ATmega32" | "ATmega323" | "ATmega324A" | "ATmega324P" | "ATmega324PA" | "ATmega325"
        | "ATmega325A" | "ATmega325P" | "ATmega325PA" | "ATmega3250" | "ATmega3250A"
        | "ATmega3250P" | "ATmega3250PA" | "ATmega328" | "ATmega328P" | "ATmega328PB"
        | "ATmega329" | "ATmega329A" | "ATmega329P" | "ATmega329PA" | "ATmega3290"
        | "ATmega3290A" | "ATmega3290P" | "ATmega3290PA" | "ATmega32C1" | "ATmega32M1"
        | "ATmega32U4" | "ATmega32U6" | "ATmega406" | "ATmega64" | "ATmega64A" | "ATmega640"
        | "ATmega644" | "ATmega644A" | "ATmega644P" | "ATmega644PA" | "ATmega645"
        | "ATmega645A" | "ATmega645P" | "ATmega6450" | "ATmega6450A" | "ATmega6450P"
        | "ATmega649" | "ATmega649A" | "ATmega649P" | "ATmega6490" | "ATmega16HVA"
        | "ATmega16HVA2" | "ATmega32HVB" | "ATmega6490A" | "ATmega6490P" | "ATmega64C1"
        | "ATmega64M1" | "ATmega64HVE" | "ATmega64HVE2" | "ATmega64RFR2" | "ATmega644RFR2"
        | "ATmega32HVBREVB" | "AT90CAN32" | "AT90CAN64" | "AT90PWM161" | "AT90PWM216"
        | "AT90PWM316" | "AT90SCR100" | "AT90USB646" | "AT90USB647" | "AT94K" | "M3000" => Avr5,

        "ATmega128" | "ATmega128A" | "ATmega1280" | "ATmega1281" | "ATmega1284" | "ATmega1284P"
        | "ATmega128RFA1" | "ATmega128RFR2" | "ATmega1284RFR2" | "AT90CAN128" | "AT90USB1286"
        | "AT90USB1287" => Avr51,

        "ATmega2560" | "ATmega2561" | "ATmega256RFR2" | "ATmega2564RFR2" => Avr6,

        "ATxmega8E5" | "ATxmega16A4" | "ATxmega16D4" | "ATxmega16E5" | "ATxmega32A4"
        | "ATxmega32C3" | "ATxmega32D3" | "ATxmega32D4" | "ATxmega16A4U" | "ATxmega16C4"
        | "ATxmega32A4U" | "ATxmega32C4" | "ATxmega32E5" => Xmega2,

        "ATtiny212" | "ATtiny214" | "ATtiny412" | "ATtiny414" | "ATtiny416" | "ATtiny417"
        | "ATtiny814" | "ATtiny816" | "ATtiny817" | "ATtiny1614" | "ATtiny1616" | "ATtiny1617"
        | "ATtiny3214" | "ATtiny3216" | "ATtiny3217" => Xmega3,

        "ATxmega64A3" | "ATxmega64D3" | "ATxmega64A3U" | "ATxmega64A4U" | "ATxmega64B1"
        | "ATxmega64B3" | "ATxmega64C3" | "ATxmega64D4" => Xmega4,

        "ATxmega64A1" | "ATxmega64A1U" => Xmega5,

        "ATxmega128A3" | "ATxmega128D3" | "ATxmega192A3" | "ATxmega192D3" | "ATxmega256A3"
        | "ATxmega256A3B" | "ATxmega256A3BU" | "ATxmega256D3" | "ATxmega128A3U"
        | "ATxmega128B1" | "ATxmega128B3" | "ATxmega128C3" | "ATxmega128D4" | "ATxmega192A3U"
        | "ATxmega192C3" | "ATxmega256A3U" | "ATxmega256C3" | "ATxmega384C3" | "ATxmega384D3" => {
            Xmega6
        }

        "ATxmega128A1" | "ATxmega128A1U" | "ATxmega128A4U" => Xmega7,

        "ATtiny4" | "ATtiny5" | "ATtiny9" | "ATtiny10" | "ATtiny102" | "ATtiny20" | "ATtiny40" => {
            Tiny
        }

        "ATA8515" | "ATA5781" | "ATA5783" | "ATA5787" | "ATA5832" | "ATA5833" | "ATA5835"
        | "ATmega324PB" | "ATtiny104" | "ATtiny80" | "ATtiny840" => Unknown,

        mcu_name => panic!("the AVR architecture name for MCU '{}' is unknown", mcu_name),
    }
}

fn c_preprocessor_name_from_mcu_name<'a>(mcu_name: &str) -> String {
    format!("__AVR_{}__", mcu_name)
}

/// Looks up extra information about a microcontroller.
pub fn lookup<T: AsRef<str>>(mcu_name: T) -> Info {
    Info {
        arch: mmcu_from_mcu_name(mcu_name.as_ref()),
        c_preprocessor_name: c_preprocessor_name_from_mcu_name(mcu_name.as_ref()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use microcontroller_names;

    #[test]
    fn atmega328_makes_sense() {
        assert_eq!(
            Info { arch: Architecture::Avr5, c_preprocessor_name: "__AVR_ATmega328__".to_string() },
            lookup("ATmega328")
        );
    }

    #[test]
    fn matches_upper_and_lowercase() {
        assert_eq!(lookup("ATtiny85"), lookup("ATtiny85"));
    }

    #[test]
    fn there_is_a_mapping_for_every_packfile() {
        for mcu_name in microcontroller_names() {
            let info = lookup(mcu_name);
            assert!(info.c_preprocessor_name.len() > 0);
        }
    }

    // This test exists so that we can always see when
    // new unknown architectures are added.
    #[test]
    fn there_is_a_constant_number_of_unknown_architectures() {
        const EXPECTED_UNKOWNS: usize = 11;

        let unknown_count = microcontroller_names()
            .iter()
            .map(self::lookup)
            .filter(|info| info.arch == Architecture::Unknown)
            .count();
        assert_eq!(EXPECTED_UNKOWNS, unknown_count);
    }
}
