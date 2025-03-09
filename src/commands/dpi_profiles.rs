use libatk_rs::prelude::*;

static DPI_STEP: u16 = 50;

#[derive(Debug, Clone, Copy)]
pub struct Dpi(u16);

impl std::fmt::Display for Dpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.dpi())
    }
}

impl From<u16> for Dpi {
    fn from(value: u16) -> Self {
        Dpi(value)
    }
}

impl Dpi {
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != 4 {
            return Err("Invalid data length");
        }

        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(data[0])
                .wrapping_sub(data[1])
                .wrapping_sub(data[2]);
        if checksum != data[3] {
            return Err("Invalid checksum");
        }

        let x_dpi = data[0];
        let dpi_ex = data[2];

        Ok(Self(
            (((u8::MAX as u16 + 1) * dpi_ex as u16 / 0x44) + (x_dpi as u16 + 1)) * DPI_STEP,
        ))
    }

    pub fn dpi(&self) -> u16 {
        self.0
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let steps = (self.dpi() / DPI_STEP) - 1;

        let x_dpi = u8::MAX & steps as u8;
        let y_dpi = x_dpi;
        let dpi_ex = (0x44 * steps / (u8::MAX as u16 + 1)) as u8;
        let checksum = u8::MAX
            & 0x55u8
                .wrapping_sub(x_dpi)
                .wrapping_sub(y_dpi)
                .wrapping_sub(dpi_ex);

        [x_dpi, y_dpi, dpi_ex, checksum]
    }
}

#[derive(Debug, Clone, Copy)]
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
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != 4 {
            return Err("Invalid data length, must be 4 bytes");
        }

        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(data[0])
                .wrapping_sub(data[1])
                .wrapping_sub(data[2]);
        if checksum != data[3] {
            return Err("Invalid checksum");
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
pub enum Profile {
    Profile1,
    Profile2,
    Profile3,
    Profile4,
    Profile5,
    Profile6,
    Profile7,
    Profile8,
}

impl Profile {
    pub fn offset(&self) -> usize {
        match self {
            Profile::Profile1 | Profile::Profile3 | Profile::Profile5 | Profile::Profile7 => 0x0,
            Profile::Profile2 | Profile::Profile4 | Profile::Profile6 | Profile::Profile8 => 0x4,
        }
    }

    pub fn dpi(&self) -> EEPROMAddress {
        match self {
            Profile::Profile1 | Profile::Profile2 => EEPROMAddress::DpiPair1,
            Profile::Profile3 | Profile::Profile4 => EEPROMAddress::DpiPair3,
            Profile::Profile5 | Profile::Profile6 => EEPROMAddress::DpiPair5,
            Profile::Profile7 | Profile::Profile8 => EEPROMAddress::DpiPair7,
        }
    }

    pub fn color(&self) -> EEPROMAddress {
        match self {
            Profile::Profile1 | Profile::Profile2 => EEPROMAddress::DpiPair1Color,
            Profile::Profile3 | Profile::Profile4 => EEPROMAddress::DpiPair3Color,
            Profile::Profile5 | Profile::Profile6 => EEPROMAddress::DpiPair5Color,
            Profile::Profile7 | Profile::Profile8 => EEPROMAddress::DpiPair7Color,
        }
    }
}

#[derive(Command)]
pub struct DpiSetting;

#[command_extension]
impl Command<DpiSetting> {
    pub fn query(profile: Profile) -> Self {
        let mut instance = Command::default();
        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(profile.dpi());
        instance.set_data_len(0x8).unwrap();

        instance
    }

    pub fn builder(self) -> CommandBuilder<DpiSetting> {
        let mut command = self;
        command.set_id(CommandId::SetEEPROM);
        CommandBuilder::new(command)
    }

    pub fn dpi(&self, profile: Profile) -> Dpi {
        Dpi::from_bytes(&self.data()[profile.offset()..profile.offset() + 0x4])
            .expect("Failed to parse DPI")
    }

    pub fn set_dpi(&mut self, profile: Profile, dpi: u16) {
        self.set_data(Dpi::from(dpi).to_bytes().as_ref(), profile.offset())
            .expect("Failed to set DPI");
    }
}

#[derive(Command)]
pub struct DpiColorSetting;

#[command_extension]
impl Command<DpiColorSetting> {
    pub fn query(profile: Profile) -> Self {
        let mut instance = Command::default();
        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(profile.color());
        instance.set_data_len(0x8).unwrap();

        instance
    }

    pub fn builder(self) -> CommandBuilder<DpiColorSetting> {
        let mut command = self;
        command.set_id(CommandId::SetEEPROM);
        CommandBuilder::new(command)
    }

    pub fn color(&self, profile: Profile) -> Color {
        Color::from_bytes(&self.data()[profile.offset()..profile.offset() + 0x4])
            .expect("Failed to parse color")
    }

    pub fn set_color(&mut self, profile: Profile, color: Color) {
        self.set_data(color.to_bytes().as_ref(), profile.offset())
            .expect("Failed to set color");
    }
}
