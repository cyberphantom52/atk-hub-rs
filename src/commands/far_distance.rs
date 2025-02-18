use atk_command::{Command, CommandId, EEPROMAddress};

#[derive(Command)]
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct FarDistanceMode {
    raw: Vec<u8>,
}

impl std::fmt::Display for FarDistanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Far Distance Mode: {}", self.far_distance_mode())
    }
}

impl FarDistanceMode {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::GetFarDistanceMode);

        command
    }

    pub fn far_distance_mode(&self) -> bool {
        self.raw[Self::base_offset()] == 0x01
    }

    pub fn set_far_distance_mode(&mut self, mode: bool) {
        self.raw[Self::base_offset()] = if mode { 0x01 } else { 0x00 };
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
