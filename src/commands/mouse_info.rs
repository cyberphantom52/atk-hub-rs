use libatk_rs::prelude::*;

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

#[derive(Command)]
pub struct MouseInfo;

#[command_extension]
impl Command<MouseInfo> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetEEPROM);
        command.set_eeprom_address(EEPROMAddress::ReportRate);
        command.set_data_len(0x6).unwrap();

        command
    }

    pub fn builder() -> CommandBuilder<MouseInfo> {
        let mut command = Command::default();

        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(EEPROMAddress::ReportRate);
        command.set_data_len(0x6).unwrap();

        CommandBuilder::new(command)
    }

    pub fn poll_rate(&self) -> PollingRate {
        self.data()[0x0].into()
    }

    pub fn set_poll_rate(&mut self, rate: PollingRate) {
        self.set_data_byte_with_checksum(rate as u8, 0x0).unwrap();
    }

    pub fn num_profile(&self) -> u8 {
        self.as_bytes()[0x2]
    }

    pub fn set_num_profile(&mut self, dpi: u8) {
        self.set_data_byte_with_checksum(dpi, 0x2).unwrap();
    }

    pub fn active_profile(&self) -> u8 {
        self.as_bytes()[0x4]
    }

    pub fn set_active_profile(&mut self, dpi: u8) {
        self.set_data_byte_with_checksum(dpi, 0x4).unwrap();
    }
}
