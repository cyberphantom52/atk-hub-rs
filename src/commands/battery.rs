use super::{Command, CommandDescriptor, CommandId};
use atk_command_derive::CommandDescriptor;

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct GetBatteryStatus;

impl std::fmt::Display for Command<GetBatteryStatus> {
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

impl Command<GetBatteryStatus> {
    pub fn query() -> Command<GetBatteryStatus> {
        let mut command = Command::default();

        command.set_id(CommandId::GetBatteryLevel);

        command
    }

    pub fn level(&self) -> u8 {
        self.as_bytes()[GetBatteryStatus::base_offset()]
    }

    pub fn charge(&self) -> u8 {
        self.as_bytes()[GetBatteryStatus::base_offset() + 0x1]
    }

    pub fn voltage(&self) -> f32 {
        self.as_bytes()[GetBatteryStatus::base_offset() + 0x2] as f32 / 10f32
    }
}
