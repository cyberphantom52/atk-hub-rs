use atk_command::{Command, CommandId, EEPROMAddress};

#[derive(Command)]
#[cmd_len(0x10)]
#[report_id(0x8)]
#[base_offset(0x5)]
pub struct Pair {
    raw: Vec<u8>,
}

impl Pair {
    pub fn pair(cid: u8, mid: u8) -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };
        command.set_id(CommandId::SetWirelessDonglePair);
        command.set_valid_data_len(0x2);
        command.set_cid(cid);
        command.set_mid(mid);

        command
    }

    fn set_cid(&mut self, cid: u8) {
        self.raw[Self::base_offset()] = cid;
    }

    fn set_mid(&mut self, mid: u8) {
        self.raw[Self::base_offset() + 0x1] = mid;
    }
}

#[derive(Command)]
#[cmd_len(0x10)]
#[report_id(0x8)]
#[base_offset(0x5)]
pub struct PairingStatus {
    raw: Vec<u8>,
}

impl std::fmt::Display for PairingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pairing Status: {} | Time Left: {}",
            self.pair_status(),
            self.pair_time_left()
        )
    }
}

impl PairingStatus {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };
        command.set_id(CommandId::GetWirelessDonglePairResult);

        command
    }

    pub fn pair_status(&self) -> u8 {
        self.raw[Self::base_offset()]
    }

    pub fn pair_time_left(&self) -> u8 {
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
#[cmd_len(0x10)]
#[report_id(0x8)]
#[base_offset(0x5)]
pub struct ExitPairing {
    raw: Vec<u8>,
}

impl ExitPairing {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };
        command.set_id(CommandId::DongleExitPair);

        command
    }
}
