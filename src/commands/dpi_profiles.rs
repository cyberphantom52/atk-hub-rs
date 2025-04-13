use libatk_rs::prelude::*;

use crate::{
    proto,
    types::{Color, Dpi},
};

#[derive(Debug, Clone, Copy)]
pub enum Gear {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Into<proto::Gear> for Gear {
    fn into(self) -> proto::Gear {
        match self {
            Gear::One => proto::Gear::One,
            Gear::Two => proto::Gear::Two,
            Gear::Three => proto::Gear::Three,
            Gear::Four => proto::Gear::Four,
            Gear::Five => proto::Gear::Five,
            Gear::Six => proto::Gear::Six,
            Gear::Seven => proto::Gear::Seven,
            Gear::Eight => proto::Gear::Eight,
        }
    }
}

impl TryFrom<proto::Gear> for Gear {
    type Error = Error;

    fn try_from(value: proto::Gear) -> Result<Self, Self::Error> {
        match value {
            proto::Gear::One => Ok(Gear::One),
            proto::Gear::Two => Ok(Gear::Two),
            proto::Gear::Three => Ok(Gear::Three),
            proto::Gear::Four => Ok(Gear::Four),
            proto::Gear::Five => Ok(Gear::Five),
            proto::Gear::Six => Ok(Gear::Six),
            proto::Gear::Seven => Ok(Gear::Seven),
            proto::Gear::Eight => Ok(Gear::Eight),
            _ => Err(Error::ParseError(format!(
                "Preset: Invalid DPI profile: {}",
                value as u8
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Pair {
    #[default]
    Pair1,
    Pair2,
    Pair3,
    Pair4,
}

impl From<Gear> for Pair {
    fn from(value: Gear) -> Self {
        match value {
            Gear::One | Gear::Two => Pair::Pair1,
            Gear::Three | Gear::Four => Pair::Pair2,
            Gear::Five | Gear::Six => Pair::Pair3,
            Gear::Seven | Gear::Eight => Pair::Pair4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Slot {
    First = 0x0,
    Second = 0x4,
}

impl From<Gear> for Slot {
    fn from(value: Gear) -> Self {
        match value {
            Gear::One | Gear::Three => Slot::First,
            Gear::Two | Gear::Four => Slot::Second,
            Gear::Five | Gear::Seven => Slot::First,
            Gear::Six | Gear::Eight => Slot::Second,
        }
    }
}

impl TryFrom<u8> for Gear {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Gear::One),
            2 => Ok(Gear::Two),
            3 => Ok(Gear::Three),
            4 => Ok(Gear::Four),
            5 => Ok(Gear::Five),
            6 => Ok(Gear::Six),
            7 => Ok(Gear::Seven),
            8 => Ok(Gear::Eight),
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
pub struct Profile {
    dpi: Dpi,
    color: Color,
}

impl Profile {
    pub fn new(dpi: Dpi, color: Color) -> Self {
        Profile { dpi, color }
    }

    pub fn dpi(&self) -> Dpi {
        self.dpi
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl std::fmt::Display for Profile {
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
