use libatk_rs::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum ConnectionType {
    #[default]
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

#[derive(Command, Default)]
pub struct DownloadData {
    encrypted_data: [u8; 4],
    cid: u8,
    mid: u8,
    device_type: ConnectionType,
}

impl DownloadData {
    pub fn encrypted_data(&self) -> &[u8; 4] {
        &self.encrypted_data
    }

    pub fn cid(&self) -> u8 {
        self.cid
    }

    pub fn mid(&self) -> u8 {
        self.mid
    }

    pub fn device_type(&self) -> ConnectionType {
        self.device_type
    }
}

#[command_extension]
impl Command<DownloadData> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::DownLoadData);
        command.set_data_len(0x8).unwrap();

        command
    }

    pub fn config(self) -> DownloadData {
        let encrypted_data: [u8; 4] = self.data()[..0x4].try_into().unwrap();
        DownloadData {
            encrypted_data,
            cid: self.data()[0x4],
            mid: self.data()[0x5],
            device_type: self.data()[0x6].into(),
        }
    }
}

#[derive(Command, Default)]
pub struct DriverStatus(u8);

#[command_extension]
impl Command<DriverStatus> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::DownLoadDriverStatus);

        command
    }

    pub fn config(self) -> DriverStatus {
        DriverStatus(self.data()[0x0])
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

    fn rf_id_3(&self) -> u8 {
        self.data()[0x1]
    }

    fn rf_id_2(&self) -> u8 {
        self.data()[0x2]
    }

    fn rf_id_1(&self) -> u8 {
        self.data()[0x3]
    }

    fn rf_id(&self) -> [u8; 3] {
        [self.rf_id_1(), self.rf_id_2(), self.rf_id_3()]
    }
}

#[derive(Command, Default)]
pub struct GetMouseCidMid(u8, u8);

impl GetMouseCidMid {
    pub fn cid(&self) -> u8 {
        self.0
    }

    pub fn mid(&self) -> u8 {
        self.1
    }
}

#[command_extension]
impl Command<GetMouseCidMid> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetMouseCIDMID);

        command
    }

    pub fn config(self) -> GetMouseCidMid {
        GetMouseCidMid(self.data()[0x0], self.data()[0x1])
    }
}

#[derive(Command, Default)]
pub struct GetMouseVersion(u8, u8);

impl GetMouseVersion {
    pub fn major(&self) -> u8 {
        self.0
    }

    pub fn minor(&self) -> u8 {
        self.1
    }
}

impl std::fmt::Display for GetMouseVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}{:02x}", self.0, self.1)
    }
}

#[command_extension]
impl Command<GetMouseVersion> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetMouseVersion);

        command
    }

    pub fn config(self) -> GetMouseVersion {
        let major = self.data()[0x0];
        let minor = self.data()[0x1];

        GetMouseVersion(major, minor)
    }
}
