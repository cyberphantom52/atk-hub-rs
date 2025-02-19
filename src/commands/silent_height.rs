use super::{Command, CommandDescriptor, CommandId, EEPROMAddress};
use atk_command_derive::CommandDescriptor;

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

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct SilentHeight;

impl std::fmt::Display for Command<SilentHeight> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mouse Silent Height: {:?}", self.silent_height())
    }
}

impl Command<SilentHeight> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetEEPROM);
        command.set_eeprom_address(EEPROMAddress::SilentHeight);
        command.set_valid_data_len(0x2);

        command
    }

    pub fn silent_height(&self) -> SilentHeightMode {
        self.as_bytes()[SilentHeight::base_offset()].into()
    }

    pub fn set_silent_height(&mut self, mode: SilentHeightMode) {
        self.set_byte_pair(mode as u8, SilentHeight::base_offset())
            .unwrap();
    }
}
