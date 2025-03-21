use libatk_rs::prelude::*;

static DPI_STEP: u16 = 50;

#[derive(Debug, Clone, Copy)]
pub struct Dpi(u16);

impl std::fmt::Display for Dpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.dpi())
    }
}

impl Default for Dpi {
    fn default() -> Self {
        Dpi(1600)
    }
}

impl From<u16> for Dpi {
    fn from(value: u16) -> Self {
        Dpi(value)
    }
}

impl TryFrom<&[u8]> for Dpi {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 4 {
            return Err(Error::ParseError(format!(
                "DPI: Invalid data length: expected 4 got {}",
                data.len()
            )));
        }

        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(data[0])
                .wrapping_sub(data[1])
                .wrapping_sub(data[2]);
        if checksum != data[3] {
            return Err(Error::ParseError("DPI: Invalid checksum".to_string()));
        }

        let x_dpi = data[0];
        let dpi_ex = data[2];

        Ok(Self(
            (((u8::MAX as u16 + 1) * dpi_ex as u16 / 0x44) + (x_dpi as u16 + 1)) * DPI_STEP,
        ))
    }
}

impl Into<[u8; 4]> for Dpi {
    fn into(self) -> [u8; 4] {
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

impl Dpi {
    pub fn new(dpi: u16) -> Self {
        Dpi(dpi)
    }

    pub fn dpi(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0xFF,
            green: 0xFF,
            blue: 0xFF,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

impl TryFrom<&[u8]> for Color {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 4 {
            return Err(Error::ParseError("Color: Invalid data length".to_string()));
        }

        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(data[0])
                .wrapping_sub(data[1])
                .wrapping_sub(data[2]);

        if checksum != data[3] {
            return Err(Error::ParseError("Color: Invalid checksum".to_string()));
        }

        Ok(Self {
            red: data[0],
            green: data[1],
            blue: data[2],
        })
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(self.red)
                .wrapping_sub(self.green)
                .wrapping_sub(self.blue);

        [self.red, self.green, self.blue, checksum]
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

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
            0 => Ok(Preset::Preset1),
            1 => Ok(Preset::Preset2),
            2 => Ok(Preset::Preset3),
            3 => Ok(Preset::Preset4),
            4 => Ok(Preset::Preset5),
            5 => Ok(Preset::Preset6),
            6 => Ok(Preset::Preset7),
            7 => Ok(Preset::Preset8),
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
