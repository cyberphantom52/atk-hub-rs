use super::{Command, CommandDescriptor, CommandId, EEPROMAddress};
use atk_command_derive::CommandDescriptor;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PollingRate {
    Hz1000 = 0x1,
    Hz500 = 0x2,
    Hz250 = 0x4,
    Hz125 = 0x8,
    Hz2000 = 0x10,
    Hz4000 = 0x20,
    Hz8000 = 0x40,
}

impl std::fmt::Display for PollingRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PollingRate::Hz1000 => write!(f, "1000Hz"),
            PollingRate::Hz500 => write!(f, "500Hz"),
            PollingRate::Hz250 => write!(f, "250Hz"),
            PollingRate::Hz125 => write!(f, "125Hz"),
            PollingRate::Hz2000 => write!(f, "2000Hz"),
            PollingRate::Hz4000 => write!(f, "4000Hz"),
            PollingRate::Hz8000 => write!(f, "8000Hz"),
        }
    }
}

impl From<u8> for PollingRate {
    fn from(value: u8) -> Self {
        match value {
            0x1 => PollingRate::Hz1000,
            0x2 => PollingRate::Hz500,
            0x4 => PollingRate::Hz250,
            0x8 => PollingRate::Hz125,
            0x10 => PollingRate::Hz2000,
            0x20 => PollingRate::Hz4000,
            0x40 => PollingRate::Hz8000,
            _ => panic!("Invalid Value"),
        }
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct MouseInfo;

impl std::fmt::Display for Command<MouseInfo> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Poll Rate: {} | Current Profile: {} | Number of Profles: {}",
            self.poll_rate(),
            self.active_profile() + 1,
            self.num_profile()
        )
    }
}

impl Command<MouseInfo> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetEEPROM);
        command.set_eeprom_address(EEPROMAddress::ReportRate);
        command.set_valid_data_len(0x6);

        command
    }

    pub fn poll_rate(&self) -> PollingRate {
        self.as_bytes()[MouseInfo::base_offset()].into()
    }

    pub fn set_poll_rate(&mut self, rate: PollingRate) {
        self.set_byte_pair(rate as u8, MouseInfo::base_offset())
            .unwrap();
    }

    pub fn num_profile(&self) -> u8 {
        self.as_bytes()[MouseInfo::base_offset() + 0x2]
    }

    pub fn set_num_profile(&mut self, dpi: u8) {
        self.set_byte_pair(dpi, MouseInfo::base_offset() + 0x2)
            .unwrap();
    }

    pub fn active_profile(&self) -> u8 {
        self.as_bytes()[MouseInfo::base_offset() + 0x4]
    }

    pub fn set_active_profile(&mut self, dpi: u8) {
        self.set_byte_pair(dpi, MouseInfo::base_offset() + 0x4)
            .unwrap();
    }
}
