use libatk_rs::prelude::*;

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
pub struct SilentHeight;

#[command_extension]
impl Command<SilentHeight> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetEEPROM);
        command.set_eeprom_address(EEPROMAddress::SilentHeight);
        command.set_data_len(0x2).unwrap();

        command
    }

    pub fn builder() -> CommandBuilder<SilentHeight> {
        let mut command = Command::default();

        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(EEPROMAddress::SilentHeight);
        command.set_data_len(0x2).unwrap();

        CommandBuilder::new(command)
    }

    pub fn silent_height(&self) -> SilentHeightMode {
        self.as_bytes()[0x0].into()
    }

    pub fn set_silent_height(&mut self, mode: SilentHeightMode) {
        self.set_data_byte_with_checksum(mode as u8, 0x0).unwrap();
    }
}
