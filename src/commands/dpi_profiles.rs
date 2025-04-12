use libatk_rs::prelude::*;

use crate::types::{Color, Dpi};

#[derive(Debug, Clone, Copy)]
pub enum Preset {
    Preset1,
    Preset2,
    Preset3,
    Preset4,
    Preset5,
    Preset6,
    Preset7,
    Preset8,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Pair {
    #[default]
    Pair1,
    Pair2,
    Pair3,
    Pair4,
}

impl From<Preset> for Pair {
    fn from(value: Preset) -> Self {
        match value {
            Preset::Preset1 | Preset::Preset2 => Pair::Pair1,
            Preset::Preset3 | Preset::Preset4 => Pair::Pair2,
            Preset::Preset5 | Preset::Preset6 => Pair::Pair3,
            Preset::Preset7 | Preset::Preset8 => Pair::Pair4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Slot {
    First = 0x0,
    Second = 0x4,
}

impl From<Preset> for Slot {
    fn from(value: Preset) -> Self {
        match value {
            Preset::Preset1 | Preset::Preset3 => Slot::First,
            Preset::Preset2 | Preset::Preset4 => Slot::Second,
            Preset::Preset5 | Preset::Preset7 => Slot::First,
            Preset::Preset6 | Preset::Preset8 => Slot::Second,
        }
    }
}

impl TryFrom<u8> for Preset {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Preset::Preset1),
            2 => Ok(Preset::Preset2),
            3 => Ok(Preset::Preset3),
            4 => Ok(Preset::Preset4),
            5 => Ok(Preset::Preset5),
            6 => Ok(Preset::Preset6),
            7 => Ok(Preset::Preset7),
            8 => Ok(Preset::Preset8),
            _ => Err(Error::ParseError(format!(
                "Preset: Invalid DPI profile: {}",
                value
            ))),
        }
    }
}

impl Pair {
    pub fn dpi_eeprom_address(&self) -> EEPROMAddress {
        match self {
            Pair::Pair1 => EEPROMAddress::DpiPair1,
            Pair::Pair2 => EEPROMAddress::DpiPair3,
            Pair::Pair3 => EEPROMAddress::DpiPair5,
            Pair::Pair4 => EEPROMAddress::DpiPair7,
        }
    }

    pub fn color_eeprom_address(&self) -> EEPROMAddress {
        match self {
            Pair::Pair1 => EEPROMAddress::DpiPair1Color,
            Pair::Pair2 => EEPROMAddress::DpiPair3Color,
            Pair::Pair3 => EEPROMAddress::DpiPair5Color,
            Pair::Pair4 => EEPROMAddress::DpiPair7Color,
        }
    }
}

impl TryFrom<EEPROMAddress> for Pair {
    type Error = &'static str;

    fn try_from(value: EEPROMAddress) -> Result<Self, Self::Error> {
        match value {
            EEPROMAddress::DpiPair1 | EEPROMAddress::DpiPair1Color => Ok(Pair::Pair1),
            EEPROMAddress::DpiPair3 | EEPROMAddress::DpiPair3Color => Ok(Pair::Pair2),
            EEPROMAddress::DpiPair5 | EEPROMAddress::DpiPair5Color => Ok(Pair::Pair3),
            EEPROMAddress::DpiPair7 | EEPROMAddress::DpiPair7Color => Ok(Pair::Pair4),
            _ => Err("Invalid EEPROM address"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gear {
    dpi: Dpi,
    color: Color,
}

impl Gear {
    pub fn new(dpi: Dpi, color: Color) -> Self {
        Gear { dpi, color }
    }

    pub fn dpi(&self) -> Dpi {
        self.dpi
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl std::fmt::Display for Gear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DPI: {} | Color: {}", self.dpi, self.color)
    }
}

#[derive(Command, Debug, Default)]
pub struct DpiPairSetting {
    _pair: Pair,
    dpi_first: Dpi,
    dpi_second: Dpi,
}

#[allow(dead_code)]
impl DpiPairSetting {
    pub fn dpi(&self, slot: Slot) -> Dpi {
        match slot {
            Slot::First => self.dpi_first,
            Slot::Second => self.dpi_second,
        }
    }

    pub fn builder(&self) -> CommandBuilder<DpiPairSetting> {
        Command::<DpiPairSetting>::builder(self._pair)
            .dpi(self.dpi(Slot::First), Slot::First)
            .dpi(self.dpi(Slot::Second), Slot::Second)
    }
}

#[command_extension]
impl Command<DpiPairSetting> {
    pub fn query(pair: Pair) -> Self {
        let mut instance = Command::default();
        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(pair.dpi_eeprom_address());
        instance.set_data_len(0x8).unwrap();

        instance
    }

    pub fn builder(pair: Pair) -> CommandBuilder<DpiPairSetting> {
        let mut command = Command::default();
        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(pair.dpi_eeprom_address());
        command.set_data_len(0x8).unwrap();

        CommandBuilder::new(command)
    }

    pub fn config(self) -> DpiPairSetting {
        let data = self.data();
        let pair = Pair::try_from(self.eeprom_address())
            .expect("Failed to parse EEPROM address to DPI pair");
        let dpi1 = Dpi::try_from(&data[0..4]).expect("Failed to parse DPI #1");
        let dpi2 = Dpi::try_from(&data[4..8]).expect("Failed to parse DPI #2");
        DpiPairSetting {
            _pair: pair,
            dpi_first: dpi1,
            dpi_second: dpi2,
        }
    }

    pub fn set_dpi(&mut self, dpi: Dpi, slot: Slot) {
        let bytes: [u8; 4] = dpi.into();
        self.set_data(&bytes, slot as usize)
            .expect("Failed to set DPI value");
    }
}

#[derive(Command, Debug, Default)]
pub struct ColorPairSetting {
    _pair: Pair,
    color_first: Color,
    color_second: Color,
}

#[allow(dead_code)]
impl ColorPairSetting {
    pub fn color(&self, slot: Slot) -> Color {
        match slot {
            Slot::First => self.color_first,
            Slot::Second => self.color_second,
        }
    }

    pub fn builder(&self) -> CommandBuilder<ColorPairSetting> {
        Command::<ColorPairSetting>::builder(self._pair)
            .color(self.color(Slot::First), Slot::First)
            .color(self.color(Slot::Second), Slot::Second)
    }
}

#[command_extension]
impl Command<ColorPairSetting> {
    pub fn query(pair: Pair) -> Self {
        let mut instance = Command::default();
        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(pair.color_eeprom_address());
        instance.set_data_len(0x8).unwrap();
        instance
    }

    pub fn builder(pair: Pair) -> CommandBuilder<ColorPairSetting> {
        let mut command = Command::default();
        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(pair.color_eeprom_address());
        command.set_data_len(0x8).unwrap();

        CommandBuilder::new(command)
    }

    pub fn config(self) -> ColorPairSetting {
        let data = self.data();
        let pair = Pair::try_from(self.eeprom_address())
            .expect("Failed to parse EEPROM address to DPI pair");
        let color1 = Color::try_from(&data[0..4]).expect("Failed to parse color #1");
        let color2 = Color::try_from(&data[4..8]).expect("Failed to parse color #2");
        ColorPairSetting {
            _pair: pair,
            color_first: color1,
            color_second: color2,
        }
    }

    pub fn set_color(&mut self, color: Color, slot: Slot) {
        let bytes: [u8; 4] = color.into();
        self.set_data(&bytes, slot as usize)
            .expect("Failed to set color value");
    }
}
