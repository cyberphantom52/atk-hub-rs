use atk_command::{Command, CommandId, EEPROMAddress};

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum SilentHeightMode {
    Off,
    OneMm,
    TwoMm,
}

impl From<u8> for SilentHeightMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => SilentHeightMode::Off,
            0x01 => SilentHeightMode::OneMm,
            0x02 => SilentHeightMode::TwoMm,
            _ => SilentHeightMode::Off,
        }
    }
}

#[derive(Command)]
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct SilentHeight {
    raw: Vec<u8>,
}

impl std::fmt::Display for SilentHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mouse Silent Height: {:?}", self.silent_height())
    }
}

impl SilentHeight {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::GetEEPROM);
        command.set_eeprom_address(EEPROMAddress::SilentHeight);
        command.set_valid_data_len(0x2);

        command
    }

    pub fn silent_height(&self) -> SilentHeightMode {
        self.raw[Self::base_offset()].into()
    }

    pub fn set_silent_height(&mut self, mode: SilentHeightMode) {
        self.set_byte_pair(mode as u8, Self::base_offset()).unwrap();
    }

    pub fn try_from(raw: &[u8]) -> Result<Self, String> {
        if raw.len() != Self::cmd_len() {
            return Err(format!(
                "Invalid command length: expected {}, got {}",
                Self::cmd_len(),
                raw.len()
            ));
        }

        Ok(Self { raw: raw.to_vec() })
    }
}
