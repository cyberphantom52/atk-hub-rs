use atk_command::{Command, CommandId, EEPROMAddress};

#[derive(Command)]
#[base_offset(0x5)]
#[report_id(0x8)]
#[cmd_len(0x10)]
pub struct BatteryStatus {
    raw: Vec<u8>,
}

impl std::fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Battery Level: {}% | Charge: {}C | Voltage: {}V",
            self.level(),
            self.charge(),
            self.voltage()
        )
    }
}

impl BatteryStatus {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        command.set_id(CommandId::GetBatteryLevel);

        command
    }

    pub fn level(&self) -> u8 {
        self.raw[Self::base_offset()]
    }

    pub fn charge(&self) -> u8 {
        self.raw[Self::base_offset() + 0x1]
    }

    pub fn voltage(&self) -> f32 {
        self.raw[Self::base_offset() + 0x2] as f32 / 10f32
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
