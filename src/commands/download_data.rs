use libatk_rs::prelude::*;

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

#[derive(Command)]
pub struct DownloadData;

#[command_extension]
impl Command<DownloadData> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::DownLoadData);
        command.set_data_len(0x8).unwrap();

        command
    }

    pub fn encrypted_data(&self) -> &[u8] {
        &self.data()[..0x4]
    }

    pub fn cid(&self) -> u8 {
        self.data()[0x4]
    }

    pub fn mid(&self) -> u8 {
        self.data()[0x5]
    }

    pub fn device_type(&self) -> ConnectionType {
        self.data()[0x6].into()
    }
}

#[derive(Command)]
pub struct DownLoadDriverStatus;

#[command_extension]
impl Command<DownLoadDriverStatus> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::DownLoadDriverStatus);

        command
    }

    pub fn driver_status(&self) -> u8 {
        self.data()[0x0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Command)]
pub struct GetWirelessMouseOnline;

#[command_extension]
impl Command<GetWirelessMouseOnline> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetWirelessMouseOnline);

        command
    }

    pub fn mouse_status(&self) -> MouseStatus {
        self.data()[0x0].into()
    }

    pub fn rf_id_3(&self) -> u8 {
        self.data()[0x1]
    }

    pub fn rf_id_2(&self) -> u8 {
        self.data()[0x2]
    }

    pub fn rf_id_1(&self) -> u8 {
        self.data()[0x3]
    }
}

#[derive(Command)]
pub struct GetMouseCidMid;

#[command_extension]
impl Command<GetMouseCidMid> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetMouseCIDMID);

        command
    }

    pub fn cid(&self) -> u8 {
        self.data()[0x0]
    }

    pub fn mid(&self) -> u8 {
        self.data()[0x1]
    }
}

#[derive(Command)]
pub struct GetMouseVersion;

#[command_extension]
impl Command<GetMouseVersion> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetMouseVersion);

        command
    }

    pub fn version(&self) -> String {
        let major = self.data()[0x0];
        let minor = self.data()[0x1];
        format!("{:02x}{:02x}", major, minor)
    }
}
