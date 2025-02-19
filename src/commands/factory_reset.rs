use super::{Command, CommandDescriptor, CommandId};
use atk_command_derive::CommandDescriptor;

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct FactoryReset;

impl Command<FactoryReset> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::RestoreFactory);

        command
    }
}
