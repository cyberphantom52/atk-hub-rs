use super::{Command, CommandDescriptor, CommandId};
use atk_command_derive::CommandDescriptor;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ConnectionType {
    Dongle1K,
    Dongle4K,
    Wired1K,
    Wired8K,
    Dongle2K,
    Dongle8K,
}

impl From<u8> for ConnectionType {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ConnectionType::Dongle1K,
            0x1 => ConnectionType::Dongle4K,
            0x2 => ConnectionType::Wired1K,
            0x3 => ConnectionType::Wired8K,
            0x4 => ConnectionType::Dongle2K,
            0x5 => ConnectionType::Dongle8K,
            _ => ConnectionType::Dongle1K,
        }
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct DownloadData;

impl std::fmt::Display for Command<DownloadData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Encrypted Data: {:X?} | CID: {} | MID: {} | Device Type: {:?}",
            self.encrypted_data(),
            self.cid(),
            self.mid(),
            self.device_type()
        )
    }
}

impl Command<DownloadData> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::DownLoadData);
        command.set_valid_data_len(0x8);

        command
    }

    pub fn encrypted_data(&self) -> &[u8] {
        &self.as_bytes()[DownloadData::base_offset()..DownloadData::base_offset() + 0x4]
    }

    pub fn set_encrypted_data(&mut self, data: &[u8; 4]) {
        self.raw_mut()[DownloadData::base_offset()..DownloadData::base_offset() + 0x4]
            .copy_from_slice(data);
    }

    pub fn cid(&self) -> u8 {
        self.as_bytes()[DownloadData::base_offset() + 0x4]
    }

    pub fn mid(&self) -> u8 {
        self.as_bytes()[DownloadData::base_offset() + 0x5]
    }

    pub fn device_type(&self) -> ConnectionType {
        self.as_bytes()[DownloadData::base_offset() + 0x6].into()
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct DownLoadDriverStatus;

impl std::fmt::Display for Command<DownLoadDriverStatus> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Driver Status: {}", self.driver_status())
    }
}

impl Command<DownLoadDriverStatus> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::DownLoadDriverStatus);

        command
    }

    pub fn driver_status(&self) -> u8 {
        self.as_bytes()[DownLoadDriverStatus::base_offset()]
    }

    pub fn set_driver_status(&mut self, status: u8) {
        self.raw_mut()[DownLoadDriverStatus::base_offset()] = status;
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MouseStatus {
    Dormant,
    Active,
}

impl From<u8> for MouseStatus {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MouseStatus::Dormant,
            0x1 => MouseStatus::Active,
            _ => MouseStatus::Dormant,
        }
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct GetWirelessMouseOnline;

impl std::fmt::Display for Command<GetWirelessMouseOnline> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mouse Status: {:?} | RF ID: {:02X} {:02X} {:02X}",
            self.mouse_status(),
            self.rf_id_3(),
            self.rf_id_2(),
            self.rf_id_1()
        )
    }
}

impl Command<GetWirelessMouseOnline> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetWirelessMouseOnline);

        command
    }

    pub fn mouse_status(&self) -> MouseStatus {
        self.as_bytes()[GetWirelessMouseOnline::base_offset()].into()
    }

    pub fn rf_id_3(&self) -> u8 {
        self.as_bytes()[GetWirelessMouseOnline::base_offset() + 0x1]
    }

    pub fn rf_id_2(&self) -> u8 {
        self.as_bytes()[GetWirelessMouseOnline::base_offset() + 0x2]
    }

    pub fn rf_id_1(&self) -> u8 {
        self.as_bytes()[GetWirelessMouseOnline::base_offset() + 0x3]
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct GetMouseCIDMID;

impl std::fmt::Display for Command<GetMouseCIDMID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CID: {} | MID: {}", self.cid(), self.mid())
    }
}

impl Command<GetMouseCIDMID> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetMouseCIDMID);

        command
    }

    pub fn cid(&self) -> u8 {
        self.as_bytes()[GetMouseCIDMID::base_offset()]
    }

    pub fn mid(&self) -> u8 {
        self.as_bytes()[GetMouseCIDMID::base_offset() + 0x1]
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct GetMouseVersion;

impl std::fmt::Display for Command<GetMouseVersion> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mouse Version: V{}", self.version())
    }
}
impl Command<GetMouseVersion> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetMouseVersion);

        command
    }

    pub fn version(&self) -> String {
        let major = self.as_bytes()[GetMouseVersion::base_offset()];
        let minor = self.as_bytes()[GetMouseVersion::base_offset() + 0x1];
        format!("{:02x}{:02x}", major, minor)
    }
}
