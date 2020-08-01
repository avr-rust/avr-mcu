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

    match mcu_name.to_lowercase().as_ref() {
        "at90s1200" | "attiny11" | "attiny12" | "attiny15" | "attiny28" => Avr1,

        "at90s2313" | "at90s2323" | "at90s2333" | "at90s2343" | "attiny22" | "attiny26"
        | "at90s4414" | "at90s4433" | "at90s4434" | "at90s8515" | "at90s8534" | "at90s8535" => Avr2,

        "ata5272" | "ata6616c" | "attiny13" | "attiny13a" | "attiny2313" | "attiny2313a"
        | "attiny24" | "attiny24a" | "attiny4313" | "attiny44" | "attiny44a" | "attiny441"
        | "attiny84" | "attiny84a" | "attiny25" | "attiny45" | "attiny85" | "attiny261"
        | "attiny261a" | "attiny461" | "attiny461a" | "attiny861" | "attiny861a" | "attiny43u"
        | "attiny87" | "attiny48" | "attiny88" | "attiny828" | "attiny841" | "at86rf401" => Avr25,

        "at43usb355" | "at76c711" => Avr3,

        "atmega103" | "at43usb320" => Avr31,

        "ata5505" | "ata6617c" | "ata664251" | "at90usb82" | "at90usb162" | "atmega8u2"
        | "atmega16u2" | "atmega32u2" | "attiny167" | "attiny1634" => Avr35,

        "ata6285" | "ata6286" | "ata6289" | "ata6612c" | "atmega8" | "atmega8a" | "atmega48"
        | "atmega48a" | "atmega48p" | "atmega48pa" | "atmega48pb" | "atmega88" | "atmega88a"
        | "atmega88p" | "atmega88pa" | "atmega88pb" | "atmega8515" | "atmega8535"
        | "atmega8hva" | "at90pwm1" | "at90pwm2" | "at90pwm2b" | "at90pwm3" | "at90pwm3b"
        | "at90pwm81" => Avr4,

        "ata5700m322" | "ata5702m322" | "ata5782" | "ata5790" | "ata5790n" | "ata5791"
        | "ata5795" | "ata5831" | "ata6613c" | "ata6614q" | "ata8210" | "ata8215" | "ata8510"
        | "atmega16" | "atmega16a" | "atmega161" | "atmega162" | "atmega163" | "atmega164a"
        | "atmega164p" | "atmega164pa" | "atmega165" | "atmega165a" | "atmega165p"
        | "atmega165pa" | "atmega168" | "atmega168a" | "atmega168p" | "atmega168pa"
        | "atmega168pb" | "atmega169" | "atmega169a" | "atmega169p" | "atmega169pa"
        | "atmega16hvb" | "atmega16hvbrevb" | "atmega16m1" | "atmega16u4" | "atmega32a"
        | "atmega32" | "atmega323" | "atmega324a" | "atmega324p" | "atmega324pa" | "atmega325"
        | "atmega325a" | "atmega325p" | "atmega325pa" | "atmega3250" | "atmega3250a"
        | "atmega3250p" | "atmega3250pa" | "atmega328" | "atmega328p" | "atmega328pb"
        | "atmega329" | "atmega329a" | "atmega329p" | "atmega329pa" | "atmega3290"
        | "atmega3290a" | "atmega3290p" | "atmega3290pa" | "atmega32c1" | "atmega32m1"
        | "atmega32u4" | "atmega32u6" | "atmega406" | "atmega64" | "atmega64a" | "atmega640"
        | "atmega644" | "atmega644a" | "atmega644p" | "atmega644pa" | "atmega645"
        | "atmega645a" | "atmega645p" | "atmega6450" | "atmega6450a" | "atmega6450p"
        | "atmega649" | "atmega649a" | "atmega649p" | "atmega6490" | "atmega16hva"
        | "atmega16hva2" | "atmega32hvb" | "atmega6490a" | "atmega6490p" | "atmega64c1"
        | "atmega64m1" | "atmega64hve" | "atmega64hve2" | "atmega64rfr2" | "atmega644rfr2"
        | "atmega32hvbrevb" | "at90can32" | "at90can64" | "at90pwm161" | "at90pwm216"
        | "at90pwm316" | "at90scr100" | "at90usb646" | "at90usb647" | "at94k" | "m3000" => Avr5,

        "atmega128" | "atmega128a" | "atmega1280" | "atmega1281" | "atmega1284" | "atmega1284p"
        | "atmega128rfa1" | "atmega128rfr2" | "atmega1284rfr2" | "at90can128" | "at90usb1286"
        | "at90usb1287" => Avr51,

        "atmega2560" | "atmega2561" | "atmega256rfr2" | "atmega2564rfr2" => Avr6,

        "atxmega8e5" | "atxmega16a4" | "atxmega16d4" | "atxmega16e5" | "atxmega32a4"
        | "atxmega32c3" | "atxmega32d3" | "atxmega32d4" | "atxmega16a4u" | "atxmega16c4"
        | "atxmega32a4u" | "atxmega32c4" | "atxmega32e5" => Xmega2,

        "attiny202" | "attiny204" | "attiny212" | "attiny214" | "attiny402" | "attiny404"
        | "attiny406" | "attiny412" | "attiny414" | "attiny416" | "attiny417" | "attiny804"
        | "attiny806" | "attiny807" | "attiny814" | "attiny816" | "attiny817" | "attiny1604"
        | "attiny1606" | "attiny1607" | "attiny1614" | "attiny1616" | "attiny1617" | "attiny3214"
        | "attiny3216" | "attiny3217" | "atmega808" | "atmega809" | "atmega1608" | "atmega1609"
        | "atmega3208" | "atmega3209" | "atmega4808" | "atmega4809" => Xmega3,

        "atxmega64a3" | "atxmega64d3" | "atxmega64a3u" | "atxmega64a4u" | "atxmega64b1"
        | "atxmega64b3" | "atxmega64c3" | "atxmega64d4" => Xmega4,

        "atxmega64a1" | "atxmega64a1u" => Xmega5,

        "atxmega128a3" | "atxmega128d3" | "atxmega192a3" | "atxmega192d3" | "atxmega256a3"
        | "atxmega256a3b" | "atxmega256a3bu" | "atxmega256d3" | "atxmega128a3u"
        | "atxmega128b1" | "atxmega128b3" | "atxmega128c3" | "atxmega128d4" | "atxmega192a3u"
        | "atxmega192c3" | "atxmega256a3u" | "atxmega256c3" | "atxmega384c3" | "atxmega384d3" => {
            Xmega6
        }

        "atxmega128a1" | "atxmega128a1u" | "atxmega128a4u" => Xmega7,

        "attiny4" | "attiny5" | "attiny9" | "attiny10" | "attiny102" | "attiny20" | "attiny40" => {
            Tiny
        }

        "ata8515" | "ata5781" | "ata5783" | "ata5787" | "ata5832" | "ata5833" | "ata5835"
        | "atmega324pb" | "attiny104" | "attiny80" | "attiny840" => Unknown,

        mcu_name => panic!("the AVR architecture name for MCU '{}' is unknown", mcu_name),
    }
}

fn c_preprocessor_name_from_mcu_name(mcu_name: &str) -> String {
    let proper_mcu_name = mcu_name
        .to_uppercase()
        .replace("XMEGA", "xmega")
        .replace("MEGA", "mega")
        .replace("TINY", "tiny");
    format!("__AVR_{}__", proper_mcu_name)
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
