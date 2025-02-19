use super::{Command, CommandDescriptor, CommandId};
use atk_command_derive::CommandDescriptor;

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct FarDistanceMode;

impl std::fmt::Display for Command<FarDistanceMode> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Far Distance Mode: {}", self.far_distance_mode())
    }
}

impl Command<FarDistanceMode> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetFarDistanceMode);

        command
    }

    pub fn far_distance_mode(&self) -> bool {
        self.as_bytes()[FarDistanceMode::base_offset()] == 0x01
    }

    pub fn set_far_distance_mode(&mut self, mode: bool) {
        self.raw_mut()[FarDistanceMode::base_offset()] = if mode { 0x01 } else { 0x00 };
    }
}
