use atk_command::{Command, CommandId, EEPROMAddress};

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
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct DownloadData {
    raw: Vec<u8>,
}

impl std::fmt::Display for DownloadData {
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

impl DownloadData {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::DownLoadData);
        command.set_valid_data_len(0x8);

        command
    }

    pub fn encrypted_data(&self) -> &[u8] {
        &self.raw[Self::base_offset()..Self::base_offset() + 0x4]
    }

    pub fn set_encrypted_data(&mut self, data: &[u8; 4]) {
        self.raw[Self::base_offset()..Self::base_offset() + 0x4].copy_from_slice(data);
    }

    pub fn cid(&self) -> u8 {
        self.raw[Self::base_offset() + 0x4]
    }

    pub fn mid(&self) -> u8 {
        self.raw[Self::base_offset() + 0x5]
    }

    pub fn device_type(&self) -> ConnectionType {
        self.raw[Self::base_offset() + 0x6].into()
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

#[derive(Command)]
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct DownLoadDriverStatus {
    raw: Vec<u8>,
}

impl std::fmt::Display for DownLoadDriverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Driver Status: {}", self.driver_status())
    }
}

impl DownLoadDriverStatus {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::DownLoadDriverStatus);

        command
    }

    pub fn driver_status(&self) -> u8 {
        self.raw[Self::base_offset()]
    }

    pub fn set_driver_status(&mut self, status: u8) {
        self.raw[Self::base_offset()] = status;
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

#[derive(Command)]
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct WirelessMouseOnline {
    raw: Vec<u8>,
}

impl std::fmt::Display for WirelessMouseOnline {
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

impl WirelessMouseOnline {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::GetWirelessMouseOnline);

        command
    }

    pub fn mouse_status(&self) -> MouseStatus {
        self.raw[Self::base_offset()].into()
    }

    pub fn rf_id_3(&self) -> u8 {
        self.raw[Self::base_offset() + 0x1]
    }

    pub fn rf_id_2(&self) -> u8 {
        self.raw[Self::base_offset() + 0x2]
    }

    pub fn rf_id_1(&self) -> u8 {
        self.raw[Self::base_offset() + 0x3]
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

#[derive(Command)]
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct MouseCidMid {
    raw: Vec<u8>,
}

impl std::fmt::Display for MouseCidMid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CID: {} | MID: {}", self.cid(), self.mid())
    }
}

impl MouseCidMid {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::GetMouseCIDMID);

        command
    }

    pub fn cid(&self) -> u8 {
        self.raw[Self::base_offset()]
    }

    pub fn mid(&self) -> u8 {
        self.raw[Self::base_offset() + 0x1]
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

#[derive(Command)]
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct MouseVersion {
    raw: Vec<u8>,
}

impl std::fmt::Display for MouseVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mouse Version: V{}", self.version())
    }
}
impl MouseVersion {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::GetMouseVersion);

        command
    }

    pub fn version(&self) -> String {
        let major = self.raw[Self::base_offset()];
        let minor = self.raw[Self::base_offset() + 0x1];
        format!("{:02x}{:02x}", major, minor)
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
