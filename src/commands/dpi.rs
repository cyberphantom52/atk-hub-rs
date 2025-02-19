use super::{Command, CommandDescriptor, CommandId, EEPROMAddress};
use atk_command_derive::CommandDescriptor;

static DPI_STEP: u16 = 50;

#[derive(Debug, Clone, Copy)]
pub struct Dpi {
    x_dpi: u8,
    y_dpi: u8,
    dpi_ex: u8,
}

impl std::fmt::Display for Dpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.dpi())
    }
}

impl Dpi {
    pub fn try_from(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != 4 {
            return Err("Invalid data length");
        }

        Ok(Self {
            x_dpi: data[0],
            y_dpi: data[1],
            dpi_ex: data[2],
        })
    }

    pub fn dpi(&self) -> u16 {
        ((0x100 * self.dpi_ex as u16 / 0x44) + (self.x_dpi as u16 + 1)) * DPI_STEP
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        [
            self.x_dpi,
            self.y_dpi,
            self.dpi_ex,
            0xff & 0x55u8
                .wrapping_sub(self.x_dpi)
                .wrapping_sub(self.y_dpi)
                .wrapping_sub(self.dpi_ex),
        ]
    }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

impl Color {
    pub fn try_from(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != 4 {
            return Err("Invalid data length");
        }

        Ok(Self {
            red: data[0],
            green: data[1],
            blue: data[2],
        })
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        [
            self.red,
            self.green,
            self.blue,
            0xff & 0x55u8
                .wrapping_sub(self.red)
                .wrapping_sub(self.green)
                .wrapping_sub(self.blue),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pair {
    Pair1,
    Pair2,
    Pair3,
    Pair4,
}

impl Pair {
    pub fn dpi(&self) -> EEPROMAddress {
        match self {
            Pair::Pair1 => EEPROMAddress::DpiPair1,
            Pair::Pair2 => EEPROMAddress::DpiPair3,
            Pair::Pair3 => EEPROMAddress::DpiPair5,
            Pair::Pair4 => EEPROMAddress::DpiPair7,
        }
    }

    pub fn color(&self) -> EEPROMAddress {
        match self {
            Pair::Pair1 => EEPROMAddress::DpiPair1Color,
            Pair::Pair2 => EEPROMAddress::DpiPair3Color,
            Pair::Pair3 => EEPROMAddress::DpiPair5Color,
            Pair::Pair4 => EEPROMAddress::DpiPair7Color,
        }
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct DpiProfile;

impl Command<DpiProfile> {
    pub fn query(pair: Pair) -> Self {
        let mut instance = Command::default();
        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(pair.dpi());
        instance.set_valid_data_len(0x8);

        instance
    }

    pub fn dpis(&self) -> Result<(Dpi, Dpi), &'static str> {
        let dpis = &self.as_bytes()[DpiProfile::base_offset()
            ..(DpiProfile::base_offset() + self.valid_data_len() as usize)];
        let dpi1 = Dpi::try_from(&dpis[0..4])?;
        let dpi2 = Dpi::try_from(&dpis[4..8])?;
        Ok((dpi1, dpi2))
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct DpiColorProfile;

impl Command<DpiColorProfile> {
    pub fn query(pair: Pair) -> Self {
        let mut instance = Command::default();
        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(pair.color());
        instance.set_valid_data_len(0x8);

        instance
    }

    pub fn colors(&self) -> Result<(Color, Color), &'static str> {
        let colors = &self.as_bytes()[DpiColorProfile::base_offset()
            ..(DpiColorProfile::base_offset() + self.valid_data_len() as usize)];
        Ok((
            Color::try_from(&colors[0..4])?,
            Color::try_from(&colors[4..8])?,
        ))
    }
}
